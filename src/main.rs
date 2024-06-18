mod env;
mod utils;

use crate::utils::{fetcher, file, formatter, http};
use std::{collections::HashMap, path::PathBuf};

use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use http_body_util::BodyExt;
use reqwest::{Client, Method, StatusCode};
use tokio;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    host: String,
    port: u16,
    address: String,
    external_address: String,
    notion_pages: Vec<String>,
    path_to_notion_map: HashMap<String, String>,
}

impl AppState {
    pub fn from_env() -> Self {
        let env = env::Env::new();

        Self {
            host: env.host.into_owned(),
            port: env.port,
            address: env.address.into_owned(),
            external_address: env.external_address.into_owned(),
            notion_pages: env.notion_pages.into_owned(),
            path_to_notion_map: env.path_to_notion_map.into_owned(),
        }
    }
}

async fn handle_index_page(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    const PATH: &str = "/";

    if !state.path_to_notion_map.contains_key(PATH) {
        return (StatusCode::NOT_FOUND, headers).into_response();
    }

    let notion_page_id = state.path_to_notion_map.get(PATH).unwrap();

    return Redirect::permanent(&format!("/{}", notion_page_id)).into_response();
}

async fn handle_path_requests(
    State(state): State<AppState>,
    Path(path): Path<String>,
    method: Method,
    headers: HeaderMap,
) -> impl IntoResponse {
    let actual_path = format!("/{}", path);

    // Check if route path has path
    if state.path_to_notion_map.contains_key(&actual_path) {
        let notion_page_id = state.path_to_notion_map.get(&actual_path).unwrap();
        return Redirect::permanent(&format!("/{}", notion_page_id)).into_response();
    }

    // Check if path is a notion page
    if state.notion_pages.contains(&path) {
        let url = format!("{}/{}", state.host, path);
        let request = Client::new().request(Method::GET, url);

        return fetcher::make_response(request, headers, &state, formatter::format_notion_page)
            .await;
    }

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

    let response = fetcher::fetch_file(&location_path).await;

    if let Err(_) = response {
        return http::response_error(StatusCode::INTERNAL_SERVER_ERROR);
    }

    if let Err(_) = file::write_file(&file_path, &response.unwrap()) {
        return http::response_error(StatusCode::INTERNAL_SERVER_ERROR);
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

    dotenv().ok();

    let state = AppState::from_env();
    let addr = format!("{}:{}", state.address, state.port);
    let app = Router::new()
        .route("/", get(handle_index_page))
        .route("/*path", get(handle_path_requests))
        .route("/_assets/*path", get(handle_assets_requests))
        .route("/api/*path", post(handle_api_request))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(addr.as_str()).await.unwrap();

    info!("Server running at http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
