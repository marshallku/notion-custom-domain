use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use reqwest::{RequestBuilder, StatusCode};
use tracing::error;

use crate::AppState;

fn build_request_header(origin_header: &HeaderMap) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let headers_to_forward = vec![
        "User-Agent",
        "Accept",
        "Accept-Language",
        "Cookie",
        "Content-Type",
        "Content-Length",
        "Connection",
        "Authorization",
    ];

    for header in headers_to_forward {
        if let Some(value) = origin_header.get(header) {
            headers.insert(header, value.clone());
        }
    }

    headers
}

fn build_response_header(origin_header: &HeaderMap, state: &AppState) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let headers_to_response = vec!["Content-Type", "Cache-Control", "Set-Cookie"];

    for header in headers_to_response {
        if let Some(value) = origin_header.get(header) {
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

    headers
}

pub async fn make_response<F>(
    request: RequestBuilder,
    request_headers: HeaderMap,
    state: &AppState,
    formatter: F,
) -> Response
where
    F: Fn(String) -> String,
{
    let response = match request
        .headers(build_request_header(&request_headers))
        .send()
        .await
    {
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
    let headers = build_response_header(&response.headers(), &state);
    let body = match response.text().await {
        Ok(body) => formatter(body),
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
