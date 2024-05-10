mod env;
mod fetcher;
mod file;
mod formatter;
mod http;

use std::path::PathBuf;

use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use http_body_util::BodyExt;
use reqwest::{Client, Method, StatusCode};
use tokio;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    host: String,
    port: u16,
    address: String,
    notion_page_id: String,
    external_address: String,
}

impl AppState {
    pub fn from_env() -> Self {
        let env = env::Env::new();

        Self {
            host: env.host.into_owned(),
            port: env.port,
            address: env.address.into_owned(),
            notion_page_id: env.notion_page_id.into_owned(),
            external_address: env.external_address.into_owned(),
        }
    }
}

async fn redirect_to_notion(State(state): State<AppState>) -> impl IntoResponse {
    Redirect::permanent(&format!("/{}", state.notion_page_id))
}

async fn handle_page_request(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let url = format!("{}/{}", state.host, state.notion_page_id);
    let request = Client::new().request(Method::GET, url);
    let response =
        fetcher::make_response(request, headers, &state, formatter::format_notion_page).await;

    if response.status() != StatusCode::OK {
        return response;
    }

    response
}

async fn handle_path_requests(
    State(state): State<AppState>,
    Path(path): Path<String>,
    method: Method,
    headers: HeaderMap,
) -> impl IntoResponse {
    let url = format!("{}/{}", state.host, path);
    let request = Client::new().request(method, url);

    fetcher::make_response(request, headers, &state, formatter::modify_notion_url).await
}

async fn handle_assets_requests(Path(path): Path<String>) -> impl IntoResponse {
    let location_path = format!("_assets/{}", path);
    let file_path = PathBuf::from(format!("cache/{}", location_path));

    if file_path.exists() {
        return http::response_file(&file_path).await;
    }

    if let Err(_) = fetcher::fetch_and_cache_file(&file_path, &location_path).await {
        return http::response_error(axum::http::StatusCode::NOT_FOUND);
    }

    http::response_file(&file_path).await
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

    fetcher::make_response(request, headers, &state, formatter::modify_notion_url).await
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
        .route("/", get(redirect_to_notion))
        .route("/*path", get(handle_path_requests))
        .route("/_assets/*path", get(handle_assets_requests))
        .route(
            &format!("/{}", state.notion_page_id),
            get(handle_page_request),
        )
        .route("/api/*path", post(handle_api_request))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(addr.as_str()).await.unwrap();

    info!("Server running at http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
