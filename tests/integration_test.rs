use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use iconator::rest_api;
use serde_json::{Value, json};
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

fn parse_json(body: &str) -> Value {
    serde_json::from_str(body).unwrap_or(json!(null))
}

// === Health Endpoint ===

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

// === JSON Endpoints (return icon ID) ===

#[tokio::test]
async fn test_file_icon_id_rust() {
    let (status, body, content_type) = send_request("/icon/file?path=./test.rs").await;
    let json = parse_json(&body);

    assert_eq!(status, StatusCode::OK);
    assert!(content_type.unwrap().contains("application/json"));
    assert_eq!(json["icon_id"], 525);
}

#[tokio::test]
async fn test_file_icon_id_javascript() {
    let (status, body, content_type) = send_request("/icon/file?path=./app.js").await;
    let json = parse_json(&body);

    assert_eq!(status, StatusCode::OK);
    assert!(content_type.unwrap().contains("application/json"));
    assert_eq!(json["icon_id"], 296);
}

#[tokio::test]
async fn test_file_icon_id_typescript() {
    let (status, body, content_type) = send_request("/icon/file?path=./index.ts").await;
    let json = parse_json(&body);

    assert_eq!(status, StatusCode::OK);
    assert!(content_type.unwrap().contains("application/json"));
    assert_eq!(json["icon_id"], 633);
}

#[tokio::test]
async fn test_file_icon_id_not_found() {
    let (status, body, _content_type) = send_request("/icon/file?path=./unknown.xyz").await;
    let json = parse_json(&body);

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(json["error"], "No icon found for file");
}

#[tokio::test]
async fn test_folder_icon_id_src() {
    let (status, body, content_type) = send_request("/icon/folder?path=./src").await;
    let json = parse_json(&body);

    assert_eq!(status, StatusCode::OK);
    assert!(content_type.unwrap().contains("application/json"));
    assert_eq!(json["icon_id"], 1054);
}

#[tokio::test]
async fn test_folder_icon_id_github() {
    let (status, body, content_type) = send_request("/icon/folder?path=./.github").await;
    let json = parse_json(&body);

    assert_eq!(status, StatusCode::OK);
    assert!(content_type.unwrap().contains("application/json"));
    assert_eq!(json["icon_id"], 862);
}

#[tokio::test]
async fn test_folder_icon_id_tests() {
    let (status, body, content_type) = send_request("/icon/folder?path=./tests").await;
    let json = parse_json(&body);

    assert_eq!(status, StatusCode::OK);
    assert!(content_type.unwrap().contains("application/json"));
    assert_eq!(json["icon_id"], 1084);
}

#[tokio::test]
async fn test_folder_icon_id_not_found() {
    let (status, body, _content_type) = send_request("/icon/folder?path=./random_folder").await;
    let json = parse_json(&body);

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(json["error"], "No icon found for folder");
}

// === SVG Endpoints (return actual SVG content) ===

#[tokio::test]
async fn test_file_icon_svg_rust() {
    let (status, body, content_type) = send_request("/icon/file/svg?path=./test.rs").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type, Some("image/svg+xml".to_string()));
    assert!(body.contains("<svg"));
}

#[tokio::test]
async fn test_file_icon_svg_javascript() {
    let (status, body, content_type) = send_request("/icon/file/svg?path=./app.js").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type, Some("image/svg+xml".to_string()));
    assert!(body.contains("<svg"));
}

#[tokio::test]
async fn test_file_icon_svg_typescript() {
    let (status, body, content_type) = send_request("/icon/file/svg?path=./index.ts").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type, Some("image/svg+xml".to_string()));
    assert!(body.contains("<svg"));
}

#[tokio::test]
async fn test_file_icon_svg_not_found() {
    let (status, body, _content_type) = send_request("/icon/file/svg?path=./unknown.xyz").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, "No icon found for file");
}

#[tokio::test]
async fn test_folder_icon_svg_src() {
    let (status, body, content_type) = send_request("/icon/folder/svg?path=./src").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type, Some("image/svg+xml".to_string()));
    assert!(body.contains("<svg"));
}

#[tokio::test]
async fn test_folder_icon_svg_github() {
    let (status, body, content_type) = send_request("/icon/folder/svg?path=./.github").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type, Some("image/svg+xml".to_string()));
    assert!(body.contains("<svg"));
}

#[tokio::test]
async fn test_folder_icon_svg_tests() {
    let (status, body, content_type) = send_request("/icon/folder/svg?path=./tests").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type, Some("image/svg+xml".to_string()));
    assert!(body.contains("<svg"));
}

#[tokio::test]
async fn test_folder_icon_svg_not_found() {
    let (status, body, _content_type) =
        send_request("/icon/folder/svg?path=./random_folder").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, "No icon found for folder");
}

// === Error Handling ===

#[tokio::test]
async fn test_missing_path_param() {
    let (status, _body, _content_type) = send_request("/icon/file").await;

    // Axum returns 400 Bad Request when required query params are missing
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_missing_path_param_svg() {
    let (status, _body, _content_type) = send_request("/icon/file/svg").await;

    // Axum returns 400 Bad Request when required query params are missing
    assert_eq!(status, StatusCode::BAD_REQUEST);
}