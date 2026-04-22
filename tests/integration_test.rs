use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use iconator::rest_api;
use serde_json::{json, Value};
use tower::ServiceExt;

async fn send_request(uri: &str) -> (StatusCode, Value) {
    let app = rest_api::router();

    let request = Request::builder()
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap_or(json!(null));

    (status, json)
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
    let (status, json) = send_request("/icon/file?path=./test.rs").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["icon_id"], 525);
    assert_eq!(json["path"], "./test.rs");
}

#[tokio::test]
async fn test_file_icon_javascript() {
    let (status, json) = send_request("/icon/file?path=./app.js").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["icon_id"], 296);
    assert_eq!(json["path"], "./app.js");
}

#[tokio::test]
async fn test_file_icon_typescript() {
    let (status, json) = send_request("/icon/file?path=./index.ts").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["icon_id"], 633);
    assert_eq!(json["path"], "./index.ts");
}

#[tokio::test]
async fn test_file_icon_not_found() {
    let (status, json) = send_request("/icon/file?path=./unknown.xyz").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(json["error"], "No icon found for file");
    assert_eq!(json["path"], "./unknown.xyz");
}

#[tokio::test]
async fn test_folder_icon_src() {
    let (status, json) = send_request("/icon/folder?path=./src").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["icon_id"], 1054);
    assert_eq!(json["path"], "./src");
}

#[tokio::test]
async fn test_folder_icon_github() {
    let (status, json) = send_request("/icon/folder?path=./.github").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["icon_id"], 862);
    assert_eq!(json["path"], "./.github");
}

#[tokio::test]
async fn test_folder_icon_tests() {
    let (status, json) = send_request("/icon/folder?path=./tests").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["icon_id"], 1084);
    assert_eq!(json["path"], "./tests");
}

#[tokio::test]
async fn test_folder_icon_not_found() {
    let (status, json) = send_request("/icon/folder?path=./random_folder").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(json["error"], "No icon found for folder");
    assert_eq!(json["path"], "./random_folder");
}

#[tokio::test]
async fn test_missing_path_param() {
    let (status, _json) = send_request("/icon/file").await;

    // Axum returns 400 Bad Request when required query params are missing
    assert_eq!(status, StatusCode::BAD_REQUEST);
}