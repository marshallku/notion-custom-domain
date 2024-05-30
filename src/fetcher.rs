use axum::{
    body::Bytes,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use reqwest::{Client, RequestBuilder, StatusCode};
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

fn error_response<E>(message: &str, error: E) -> Response
where
    E: std::fmt::Debug,
{
    error!("{}: {:?}", message, error);
    (StatusCode::INTERNAL_SERVER_ERROR, message.to_string()).into_response()
}

pub async fn make_response<F>(
    request: RequestBuilder,
    request_headers: HeaderMap,
    state: &AppState,
    formatter: F,
) -> Response
where
    F: Fn(String, &AppState) -> String,
{
    let response = match request
        .headers(build_request_header(&request_headers))
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("Error fetching data: {:?}", e);
            return error_response("Failed to fetch data", e);
        }
    };
    let status = response.status();
    let headers = build_response_header(&response.headers(), &state);

    // Check if it's an image response
    if let Some(content_type) = response.headers().get("Content-Type") {
        if content_type.to_str().map_or(false, |t| t.contains("image")) {
            return (status, headers, response.bytes().await.unwrap()).into_response();
        }
    }

    let body = match response.text().await {
        Ok(body) => formatter(body, &state),
        Err(e) => {
            error!("Error reading response body: {:?}", e);
            return error_response("Failed to read response body", e);
        }
    };

    (status, headers, body).into_response()
}

pub async fn fetch_file(path: &str) -> Result<Bytes, reqwest::Error> {
    let url = format!("https://www.notion.so/{}", path);

    match Client::new().get(&url).send().await {
        Ok(response) => response.bytes().await,
        Err(err) => {
            error!("Failed to fetch {}", url);
            return Err(err);
        }
    }
}
