mod env;
mod fetcher;

use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use http_body_util::BodyExt;
use reqwest::{Client, Method};
use tokio;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    host: String,
    port: u16,
    address: String,
    notion_page_id: String,
}

impl AppState {
    pub fn from_env() -> Self {
        let env = env::Env::new();

        Self {
            host: env.host.into_owned(),
            port: env.port,
            address: env.address.into_owned(),
            notion_page_id: env.notion_page_id.into_owned(),
        }
    }
}

async fn handle_request(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    let url = format!("{}/{}", state.host, state.notion_page_id);
    let request = Client::new().request(Method::GET, url);

    fetcher::make_response(request, headers, &state).await
}

async fn handle_path_requests(
    State(state): State<AppState>,
    Path(path): Path<String>,
    method: Method,
    headers: HeaderMap,
) -> impl IntoResponse {
    let url = format!("{}/{}", state.host, path);
    let request = Client::new().request(method, url);

    fetcher::make_response(request, headers, &state).await
}

async fn handle_api_request(
    State(state): State<AppState>,
    Path(path): Path<String>,
    method: Method,
    headers: HeaderMap,
    body: Body,
) -> impl IntoResponse {
    let url = format!("{}/api/{}", state.host, path);
    let request_body = match body.collect().await {
        Ok(request_body) => reqwest::Body::from(request_body.to_bytes()),
        Err(_) => reqwest::Body::from("".as_bytes()),
    };
    let request = Client::new().request(method, url).body(request_body);

    fetcher::make_response(request, headers, &state).await
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let state = AppState::from_env();
    let addr = format!("{}:{}", state.address, state.port);
    let app = Router::new()
        .route("/", get(handle_request))
        .route("/*path", get(handle_path_requests))
        .route("/api/*path", post(handle_api_request))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(addr.as_str()).await.unwrap();

    info!("Server running at http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
