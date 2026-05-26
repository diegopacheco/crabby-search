use std::sync::RwLock;

use crate::engine::SearchEngine;

pub struct AppState {
    pub engine: RwLock<SearchEngine>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            engine: RwLock::new(SearchEngine::new()),
        }
    }
}
