mod env;

use axum::{extract::State, response::IntoResponse, routing::get, Router};
use tokio;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    host: String,
    port: u16,
    address: String,
    notion_page_url: String,
}

impl AppState {
    pub fn from_env() -> Self {
        let env = env::Env::new();

        Self {
            host: env.host.into_owned(),
            port: env.port,
            address: env.address.into_owned(),
            notion_page_url: env.notion_page_url.into_owned(),
        }
    }
}

async fn handle_request(State(state): State<AppState>) -> impl IntoResponse {
    state.notion_page_url.to_string().into_response()
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
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(addr.as_str()).await.unwrap();

    info!("Server running at http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
