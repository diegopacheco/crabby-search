use super::document::Document;
use super::index::InvertedIndex;
use super::search::{self, SearchResult};
use super::store::DocumentStore;
use super::tokenizer::tokenize;

pub struct IndexStats {
    pub document_count: usize,
    pub term_count: usize,
    pub average_length: f64,
    pub top_terms: Vec<(String, usize)>,
}

pub struct SearchEngine {
    store: DocumentStore,
    index: InvertedIndex,
}

impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {
            store: DocumentStore::new(),
            index: InvertedIndex::new(),
        }
    }

    pub fn index_document(&mut self, title: String, content: String) -> Document {
        let tokens = tokenize(&content);
        let document = self.store.insert(title, content);
        self.index.add(document.id, &tokens);
        document
    }

    pub fn delete_document(&mut self, id: u64) -> bool {
        if self.store.remove(id).is_some() {
            self.index.remove(id);
            true
        } else {
            false
        }
    }

    pub fn documents(&self) -> Vec<&Document> {
        self.store.all()
    }

    pub fn document(&self, id: u64) -> Option<&Document> {
        self.store.get(id)
    }

    pub fn document_length(&self, id: u64) -> u32 {
        self.index.document_length(id)
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        search::search(&self.store, &self.index, query, limit)
    }

    pub fn stats(&self) -> IndexStats {
        IndexStats {
            document_count: self.index.document_count(),
            term_count: self.index.term_count(),
            average_length: self.index.average_length(),
            top_terms: self.index.top_terms(50),
        }
    }
}
