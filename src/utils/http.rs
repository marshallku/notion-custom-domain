use axum::{
    body::Body,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use std::path::PathBuf;
use tokio_util::io::ReaderStream;

const YEAR_TO_SECONDS: u32 = 31536000;

pub fn get_cache_header(age: u32) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let cache_age = if age <= 0 {
        "no-cache".to_string()
    } else {
        format!("public, max-age={}", age)
    };

    headers.insert("Cache-Control", cache_age.parse().unwrap());

    headers
}

pub fn response_error(status_code: StatusCode) -> Response {
    (status_code, get_cache_header(0)).into_response()
}

pub async fn response_file(file_path: &PathBuf) -> Response {
    let file = match tokio::fs::File::open(file_path).await {
        Ok(file) => file,
        Err(_) => {
            return response_error(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    (get_cache_header(YEAR_TO_SECONDS), body).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt;
    use std::fs::File;
    use std::io::prelude::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_response_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(file_path.clone())
            .and_then(|file| Ok(file))
            .unwrap();

        file.write_all(b"test").unwrap();

        let response = response_file(&file_path).await;
        let body = response.collect().await.unwrap().to_bytes();

        assert_eq!(body, "test".as_bytes());
    }

    #[test]
    fn test_get_cache_header() {
        let headers = get_cache_header(0);
        let cache_control = headers.get("Cache-Control").unwrap().to_str().unwrap();

        assert_eq!(cache_control, "no-cache");

        let headers = get_cache_header(100);
        let cache_control = headers.get("Cache-Control").unwrap().to_str().unwrap();

        assert_eq!(cache_control, "public, max-age=100");
    }

    #[test]
    fn test_response_error() {
        let response = response_error(StatusCode::NOT_FOUND);
        let status = response.status();

        assert_eq!(status, StatusCode::NOT_FOUND);
    }
}
