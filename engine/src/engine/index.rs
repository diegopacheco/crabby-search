use std::collections::HashMap;

pub struct InvertedIndex {
    postings: HashMap<String, HashMap<u64, u32>>,
    doc_lengths: HashMap<u64, u32>,
    total_length: u64,
}

impl InvertedIndex {
    pub fn new() -> Self {
        InvertedIndex {
            postings: HashMap::new(),
            doc_lengths: HashMap::new(),
            total_length: 0,
        }
    }

    pub fn add(&mut self, doc_id: u64, tokens: &[String]) {
        let mut frequencies: HashMap<&String, u32> = HashMap::new();
        for token in tokens {
            *frequencies.entry(token).or_insert(0) += 1;
        }
        for (token, frequency) in frequencies {
            self.postings
                .entry(token.clone())
                .or_insert_with(HashMap::new)
                .insert(doc_id, frequency);
        }
        self.doc_lengths.insert(doc_id, tokens.len() as u32);
        self.total_length += tokens.len() as u64;
    }

    pub fn remove(&mut self, doc_id: u64) {
        if let Some(length) = self.doc_lengths.remove(&doc_id) {
            self.total_length -= length as u64;
        }
        let mut empty_terms = Vec::new();
        for (term, posting) in self.postings.iter_mut() {
            posting.remove(&doc_id);
            if posting.is_empty() {
                empty_terms.push(term.clone());
            }
        }
        for term in empty_terms {
            self.postings.remove(&term);
        }
    }

    pub fn postings_for(&self, term: &str) -> Option<&HashMap<u64, u32>> {
        self.postings.get(term)
    }

    pub fn document_frequency(&self, term: &str) -> usize {
        self.postings.get(term).map(|p| p.len()).unwrap_or(0)
    }

    pub fn document_length(&self, doc_id: u64) -> u32 {
        self.doc_lengths.get(&doc_id).copied().unwrap_or(0)
    }

    pub fn document_count(&self) -> usize {
        self.doc_lengths.len()
    }

    pub fn term_count(&self) -> usize {
        self.postings.len()
    }

    pub fn average_length(&self) -> f64 {
        if self.doc_lengths.is_empty() {
            0.0
        } else {
            self.total_length as f64 / self.doc_lengths.len() as f64
        }
    }

    pub fn top_terms(&self, limit: usize) -> Vec<(String, usize)> {
        let mut terms: Vec<(String, usize)> = self
            .postings
            .iter()
            .map(|(term, posting)| (term.clone(), posting.len()))
            .collect();
        terms.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        terms.truncate(limit);
        terms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens(words: &[&str]) -> Vec<String> {
        words.iter().map(|w| w.to_string()).collect()
    }

    #[test]
    fn counts_documents_and_terms() {
        let mut index = InvertedIndex::new();
        index.add(1, &tokens(&["rust", "rust", "search"]));
        index.add(2, &tokens(&["search", "engine"]));
        assert_eq!(index.document_count(), 2);
        assert_eq!(index.term_count(), 3);
        assert_eq!(index.document_frequency("search"), 2);
        assert_eq!(index.document_frequency("rust"), 1);
    }

    #[test]
    fn remove_clears_empty_terms() {
        let mut index = InvertedIndex::new();
        index.add(1, &tokens(&["only", "here"]));
        index.remove(1);
        assert_eq!(index.document_count(), 0);
        assert_eq!(index.term_count(), 0);
        assert_eq!(index.average_length(), 0.0);
    }
}
