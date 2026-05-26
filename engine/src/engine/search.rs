use std::cmp::Ordering;
use std::collections::HashMap;

use super::index::InvertedIndex;
use super::store::DocumentStore;
use super::tokenizer::tokenize;

const K1: f64 = 1.2;
const B: f64 = 0.75;

pub struct SearchResult {
    pub id: u64,
    pub title: String,
    pub score: f64,
    pub snippet: String,
}

pub fn search(
    store: &DocumentStore,
    index: &InvertedIndex,
    query: &str,
    limit: usize,
) -> Vec<SearchResult> {
    let terms = tokenize(query);
    if terms.is_empty() {
        return Vec::new();
    }

    let total_docs = index.document_count() as f64;
    let average_length = index.average_length();
    let mut scores: HashMap<u64, f64> = HashMap::new();

    for term in &terms {
        let document_frequency = index.document_frequency(term);
        if document_frequency == 0 {
            continue;
        }
        let idf = (((total_docs - document_frequency as f64 + 0.5)
            / (document_frequency as f64 + 0.5))
            + 1.0)
            .ln();
        if let Some(posting) = index.postings_for(term) {
            for (&doc_id, &frequency) in posting {
                let term_frequency = frequency as f64;
                let length = index.document_length(doc_id) as f64;
                let normalizer = if average_length > 0.0 {
                    average_length
                } else {
                    1.0
                };
                let denominator =
                    term_frequency + K1 * (1.0 - B + B * length / normalizer);
                let contribution = idf * (term_frequency * (K1 + 1.0)) / denominator;
                *scores.entry(doc_id).or_insert(0.0) += contribution;
            }
        }
    }

    let mut results: Vec<SearchResult> = scores
        .into_iter()
        .filter_map(|(id, score)| {
            store.get(id).map(|document| SearchResult {
                id,
                title: document.title.clone(),
                score,
                snippet: snippet(&document.content, &terms),
            })
        })
        .collect();

    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(Ordering::Equal)
            .then(a.id.cmp(&b.id))
    });
    results.truncate(limit);
    results
}

fn snippet(content: &str, terms: &[String]) -> String {
    let words: Vec<&str> = content.split_whitespace().collect();
    if words.is_empty() {
        return String::new();
    }
    let lowered: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();
    let mut hit = 0usize;
    'outer: for (position, word) in lowered.iter().enumerate() {
        for term in terms {
            if word.contains(term.as_str()) {
                hit = position;
                break 'outer;
            }
        }
    }
    let start = hit.saturating_sub(15);
    let end = (start + 30).min(words.len());
    let mut snippet = words[start..end].join(" ");
    if start > 0 {
        snippet = format!("... {}", snippet);
    }
    if end < words.len() {
        snippet = format!("{} ...", snippet);
    }
    snippet
}

#[cfg(test)]
mod tests {
    use super::super::engine::SearchEngine;

    #[test]
    fn ranks_most_relevant_document_first() {
        let mut engine = SearchEngine::new();
        engine.index_document(
            "rust".to_string(),
            "rust is a systems language with rust ownership and rust safety".to_string(),
        );
        engine.index_document(
            "cooking".to_string(),
            "a recipe for soup with carrots and onions".to_string(),
        );
        engine.index_document(
            "mixed".to_string(),
            "this article mentions rust once near the end".to_string(),
        );

        let results = engine.search("rust", 10);
        assert_eq!(results.first().map(|r| r.title.as_str()), Some("rust"));
        assert!(results.iter().all(|r| r.title != "cooking"));
    }

    #[test]
    fn empty_query_returns_nothing() {
        let mut engine = SearchEngine::new();
        engine.index_document("a".to_string(), "some content here".to_string());
        assert!(engine.search("   ", 10).is_empty());
    }
}
