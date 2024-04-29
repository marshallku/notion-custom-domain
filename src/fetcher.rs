use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use reqwest::{RequestBuilder, StatusCode};
use tracing::error;

use crate::AppState;

pub async fn make_response(
    request: RequestBuilder,
    request_headers: HeaderMap,
    state: &AppState,
) -> Response {
    let mut filtered_headers = HeaderMap::new();
    let headers_to_send = vec![
        "User-Agent",
        "Accept",
        "Accept-Language",
        "Cookie",
        "Content-Type",
        "Content-Length",
        "Connection",
        "Authorization",
    ];

    for header in headers_to_send {
        if let Some(value) = request_headers.get(header) {
            filtered_headers.insert(header, value.clone());
        }
    }

    let response = match request.headers(filtered_headers).send().await {
        Ok(response) => response,
        Err(e) => {
            error!("Error fetching data: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch data".to_string(),
            )
                .into_response();
        }
    };
    let status = response.status();

    let mut headers = HeaderMap::new();
    let headers_to_clone = vec!["Content-Type", "Cache-Control", "Set-Cookie"];

    for header in headers_to_clone {
        if let Some(value) = response.headers().get(header) {
            if header == "Set-Cookie" {
                let value = value
                    .to_str()
                    .unwrap()
                    .replace("notion.site", &state.address)
                    .replace("www.notion.so", &state.address);

                headers.insert(header, value.parse().unwrap());
            } else {
                headers.insert(header, value.clone());
            }
        }
    }

    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());

    let body = match response.text().await {
        Ok(body) => body
            .replace("//www.notion.so", &format!("//{}", state.address))
            .replace("//notion.so", &format!("//{}", state.address)),
        Err(e) => {
            error!("Error reading response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse body".to_string(),
            )
                .into_response();
        }
    };

    (status, headers, body).into_response()
}
