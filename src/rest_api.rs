use axum::{
    Json, Router,
    extract::Query,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};

use crate::file_loader::load_icon_svg;
use crate::icon_resolver::{get_icon_for_file, get_icon_for_folder};

#[derive(Debug, Deserialize)]
struct PathQuery {
    path: String,
}

#[derive(Debug, Serialize)]
struct IconResponse {
    icon_id: u64,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

// === JSON Endpoints (return icon ID) ===

async fn file_icon_id(Query(query): Query<PathQuery>) -> Response {
    match get_icon_for_file(&query.path) {
        Some(icon_id) => (StatusCode::OK, Json(IconResponse { icon_id })).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "No icon found for file".to_string(),
            }),
        )
            .into_response(),
    }
}

async fn folder_icon_id(Query(query): Query<PathQuery>) -> Response {
    match get_icon_for_folder(&query.path) {
        Some(icon_id) => (StatusCode::OK, Json(IconResponse { icon_id })).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "No icon found for folder".to_string(),
            }),
        )
            .into_response(),
    }
}

// === SVG Endpoints (return actual SVG content) ===

fn svg_response(svg_content: String) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/svg+xml")],
        svg_content,
    )
        .into_response()
}

fn not_found_response(message: &str) -> Response {
    (StatusCode::NOT_FOUND, message.to_string()).into_response()
}

async fn file_icon_svg(Query(query): Query<PathQuery>) -> Response {
    match get_icon_for_file(&query.path) {
        Some(icon_id) => match load_icon_svg(icon_id) {
            Ok(svg) => svg_response(svg),
            Err(_) => not_found_response("Icon file not found on disk"),
        },
        None => not_found_response("No icon found for file"),
    }
}

async fn folder_icon_svg(Query(query): Query<PathQuery>) -> Response {
    match get_icon_for_folder(&query.path) {
        Some(icon_id) => match load_icon_svg(icon_id) {
            Ok(svg) => svg_response(svg),
            Err(_) => not_found_response("Icon file not found on disk"),
        },
        None => not_found_response("No icon found for folder"),
    }
}

// === Health Endpoint ===

async fn health() -> impl IntoResponse {
    StatusCode::OK
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        // JSON endpoints (return icon ID)
        .route("/icon/file", get(file_icon_id))
        .route("/icon/folder", get(folder_icon_id))
        // SVG endpoints (return actual SVG content)
        .route("/icon/file/svg", get(file_icon_svg))
        .route("/icon/folder/svg", get(folder_icon_svg))
}