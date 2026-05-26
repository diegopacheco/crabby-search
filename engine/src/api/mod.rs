mod dto;
mod handlers;
mod state;

use actix_web::web;

pub use state::AppState;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/health").route(web::get().to(handlers::health)))
        .service(
            web::resource("/api/documents")
                .route(web::post().to(handlers::create_document))
                .route(web::get().to(handlers::list_documents)),
        )
        .service(
            web::resource("/api/documents/{id}")
                .route(web::get().to(handlers::get_document))
                .route(web::delete().to(handlers::delete_document)),
        )
        .service(web::resource("/api/search").route(web::get().to(handlers::search)))
        .service(web::resource("/api/index").route(web::get().to(handlers::index_stats)));
}
