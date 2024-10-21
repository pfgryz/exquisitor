use axum::Router;
use axum::routing::get;

pub mod server;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry();

    let app = Router::new()
        .route("/", get(server::index::render));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}

