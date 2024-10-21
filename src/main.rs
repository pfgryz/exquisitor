use axum::{Extension, Router};
use axum::routing::get;
use sqlx::SqlitePool;
use tower_http::services::ServeDir;

pub mod server;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry();

    let pool = SqlitePool::connect("sqlite://exquisitor.db")
        .await
        .unwrap();

    let serve_dir_from_assets = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(server::index::render))
        .nest_service("/assets", serve_dir_from_assets)
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}

