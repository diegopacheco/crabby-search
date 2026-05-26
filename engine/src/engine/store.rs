use std::collections::HashMap;

use super::document::Document;

pub struct DocumentStore {
    documents: HashMap<u64, Document>,
    next_id: u64,
}

impl DocumentStore {
    pub fn new() -> Self {
        DocumentStore {
            documents: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn insert(&mut self, title: String, content: String) -> Document {
        let id = self.next_id;
        self.next_id += 1;
        let document = Document { id, title, content };
        self.documents.insert(id, document.clone());
        document
    }

    pub fn get(&self, id: u64) -> Option<&Document> {
        self.documents.get(&id)
    }

    pub fn remove(&mut self, id: u64) -> Option<Document> {
        self.documents.remove(&id)
    }

    pub fn all(&self) -> Vec<&Document> {
        let mut documents: Vec<&Document> = self.documents.values().collect();
        documents.sort_by_key(|d| d.id);
        documents
    }
}
