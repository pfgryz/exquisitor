use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{extract, middleware, Extension, Router};
use server::routes;
use server::routes::errors;
use sqlx::SqlitePool;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Registry};

pub mod server;

#[tokio::main]
async fn main() {
    // Environment variables
    dotenv::dotenv().ok();

    // Logging
    tracing_subscriber::registry();

    let subscriber = Registry::default().with(fmt::layer().with_level(true));

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");

    // SQLite database
    let pool = SqlitePool::connect("sqlite://exquisitor.db").await.unwrap();

    // Static files
    let serve_dir_from_assets = ServeDir::new("static");

    // Main router
    let app = Router::new()
        .route("/", get(routes::index::render))
        .route("/search", get(routes::search::render))
        .route("/experiment/:id", get(routes::experiment::render))
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
