use crate::routes::errors;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{extract, middleware, Extension, Router};
use dotenv::dotenv;
use sqlx::SqlitePool;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Registry};
use crate::db::OrderStatus;

mod db;
mod routes;
mod templates;

#[tokio::main]
async fn main() {
    // Environment variables
    dotenv().ok();

    // Tracing
    tracing_subscriber::registry();

    let subscriber = Registry::default().with(fmt::layer().with_level(true));
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");

    // SQLite database
    let pool = SqlitePool::connect("sqlite://./exquisitor.db")
        .await
        .unwrap();

    // Static files
    let serve_dir_from_assets = ServeDir::new("static");

    // db::create_experiment(&pool, "Basic".into(), "/tmp/0.fasta".into(), OrderStatus::Queued).await.expect("");

    // Main router
    let app = Router::new()
        .route("/", get(routes::index::render))
        .route("/search", get(routes::search::render))
        //.route("/order/add", get(routes::order::add))
        .route("/order/:id", get(routes::order::render))
        .nest_service("/assets", serve_dir_from_assets)
        .fallback(errors::handle_not_found)
        .layer(Extension(pool))
        .layer(middleware::from_fn(log_request));

    // Listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn log_request(request: extract::Request, next: Next) -> impl IntoResponse {
    let method = request.method().clone();
    let path = request.uri().to_string();

    let response = next.run(request).await;

    info!("{} {} {}", method, path, &response.status());

    response
}
