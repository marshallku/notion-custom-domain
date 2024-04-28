use axum::{response::IntoResponse, routing::get, Router};
use tokio;
use tracing::info;

async fn handle_request() -> impl IntoResponse {
    "Hello, World!".into_response()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let addr = format!("{}:{}", "127.0.0.1", 8099);
    let app = Router::new().route("/", get(handle_request));

    let listener = tokio::net::TcpListener::bind(addr.as_str()).await.unwrap();

    info!("Server running at http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
