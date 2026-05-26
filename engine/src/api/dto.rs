use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateDocumentRequest {
    #[serde(default)]
    pub title: Option<String>,
    pub content: String,
}

#[derive(Deserialize)]
pub struct SearchParams {
    #[serde(default)]
    pub q: String,
    pub limit: Option<usize>,
}

#[derive(Serialize)]
pub struct DocumentSummary {
    pub id: u64,
    pub title: String,
    pub length: u32,
    pub preview: String,
}

#[derive(Serialize)]
pub struct SearchHit {
    pub id: u64,
    pub title: String,
    pub score: f64,
    pub snippet: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub query: String,
    pub count: usize,
    pub results: Vec<SearchHit>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TermStat {
    pub term: String,
    pub document_frequency: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsResponse {
    pub document_count: usize,
    pub term_count: usize,
    pub average_length: f64,
    pub top_terms: Vec<TermStat>,
}
