use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use iconator::rest_api;
use tower::ServiceExt;

async fn send_request(uri: &str) -> (StatusCode, String, Option<String>) {
    let app = rest_api::router();

    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let content_type = response
        .headers()
        .get("content-type")
        .map(|v| v.to_str().unwrap().to_string());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    (status, body_str, content_type)
}

#[tokio::test]
async fn test_health_endpoint() {
    let app = rest_api::router();

    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_file_icon_rust() {
    let (status, body, content_type) = send_request("/icon/file?path=./test.rs").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type, Some("image/svg+xml".to_string()));
    assert!(body.contains("<svg"));
}

#[tokio::test]
async fn test_file_icon_javascript() {
    let (status, body, content_type) = send_request("/icon/file?path=./app.js").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type, Some("image/svg+xml".to_string()));
    assert!(body.contains("<svg"));
}

#[tokio::test]
async fn test_file_icon_typescript() {
    let (status, body, content_type) = send_request("/icon/file?path=./index.ts").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type, Some("image/svg+xml".to_string()));
    assert!(body.contains("<svg"));
}

#[tokio::test]
async fn test_file_icon_not_found() {
    let (status, body, _content_type) = send_request("/icon/file?path=./unknown.xyz").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, "No icon found for file");
}

#[tokio::test]
async fn test_folder_icon_src() {
    let (status, body, content_type) = send_request("/icon/folder?path=./src").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type, Some("image/svg+xml".to_string()));
    assert!(body.contains("<svg"));
}

#[tokio::test]
async fn test_folder_icon_github() {
    let (status, body, content_type) = send_request("/icon/folder?path=./.github").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type, Some("image/svg+xml".to_string()));
    assert!(body.contains("<svg"));
}

#[tokio::test]
async fn test_folder_icon_tests() {
    let (status, body, content_type) = send_request("/icon/folder?path=./tests").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type, Some("image/svg+xml".to_string()));
    assert!(body.contains("<svg"));
}

#[tokio::test]
async fn test_folder_icon_not_found() {
    let (status, body, _content_type) = send_request("/icon/folder?path=./random_folder").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, "No icon found for folder");
}

#[tokio::test]
async fn test_missing_path_param() {
    let (status, _body, _content_type) = send_request("/icon/file").await;

    // Axum returns 400 Bad Request when required query params are missing
    assert_eq!(status, StatusCode::BAD_REQUEST);
}
