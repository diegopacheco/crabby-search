mod api;
mod engine;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

use api::AppState;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("CRABBY_PORT").unwrap_or_else(|_| "7700".to_string());
    let addr = format!("127.0.0.1:{}", port);
    let state = web::Data::new(AppState::new());
    println!("crabby-search engine listening on {}", addr);
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Cors::permissive())
            .configure(api::configure)
    })
    .bind(&addr)?
    .run()
    .await
}
