use actix_web::{HttpResponse, Responder, web};
use serde_json::json;

use super::dto::{
    CreateDocumentRequest, DocumentDetail, DocumentSummary, SearchHit, SearchParams,
    SearchResponse, StatsResponse, TermStat,
};
use super::state::AppState;

pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

pub async fn create_document(
    state: web::Data<AppState>,
    body: web::Json<CreateDocumentRequest>,
) -> impl Responder {
    if body.content.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({ "error": "content is required" }));
    }
    let title = body
        .title
        .clone()
        .filter(|t| !t.trim().is_empty())
        .unwrap_or_else(|| "untitled".to_string());

    let mut engine = state.engine.write().unwrap();
    let document = engine.index_document(title, body.content.clone());
    let length = engine.document_length(document.id);
    HttpResponse::Created().json(DocumentSummary {
        id: document.id,
        title: document.title.clone(),
        length,
        preview: preview(&document.content),
    })
}

pub async fn list_documents(state: web::Data<AppState>) -> impl Responder {
    let engine = state.engine.read().unwrap();
    let documents: Vec<DocumentSummary> = engine
        .documents()
        .iter()
        .map(|document| DocumentSummary {
            id: document.id,
            title: document.title.clone(),
            length: engine.document_length(document.id),
            preview: preview(&document.content),
        })
        .collect();
    HttpResponse::Ok().json(documents)
}

pub async fn get_document(state: web::Data<AppState>, path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let engine = state.engine.read().unwrap();
    match engine.document(id) {
        Some(document) => HttpResponse::Ok().json(DocumentDetail {
            id: document.id,
            title: document.title.clone(),
            content: document.content.clone(),
            length: engine.document_length(document.id),
        }),
        None => HttpResponse::NotFound().json(json!({ "error": "not found" })),
    }
}

pub async fn delete_document(state: web::Data<AppState>, path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    if state.engine.write().unwrap().delete_document(id) {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(json!({ "error": "not found" }))
    }
}

pub async fn search(state: web::Data<AppState>, params: web::Query<SearchParams>) -> impl Responder {
    let limit = params.limit.unwrap_or(20);
    let engine = state.engine.read().unwrap();
    let hits: Vec<SearchHit> = engine
        .search(&params.q, limit)
        .into_iter()
        .map(|result| SearchHit {
            id: result.id,
            title: result.title,
            score: round(result.score, 4),
            snippet: result.snippet,
        })
        .collect();
    HttpResponse::Ok().json(SearchResponse {
        query: params.q.clone(),
        count: hits.len(),
        results: hits,
    })
}

pub async fn index_stats(state: web::Data<AppState>) -> impl Responder {
    let engine = state.engine.read().unwrap();
    let stats = engine.stats();
    HttpResponse::Ok().json(StatsResponse {
        document_count: stats.document_count,
        term_count: stats.term_count,
        average_length: round(stats.average_length, 2),
        top_terms: stats
            .top_terms
            .into_iter()
            .map(|(term, document_frequency)| TermStat {
                term,
                document_frequency,
            })
            .collect(),
    })
}

fn preview(content: &str) -> String {
    let trimmed = content.trim();
    let mut preview: String = trimmed.chars().take(160).collect();
    if trimmed.chars().count() > 160 {
        preview.push_str("...");
    }
    preview
}

fn round(value: f64, decimals: u32) -> f64 {
    let factor = 10f64.powi(decimals as i32);
    (value * factor).round() / factor
}
