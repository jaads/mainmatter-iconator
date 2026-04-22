use axum::{
    Router,
    extract::Query,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::Deserialize;

use crate::file_loader::load_icon_svg;
use crate::icon_resolver::{get_icon_for_file, get_icon_for_folder};

#[derive(Debug, Deserialize)]
struct PathQuery {
    path: String,
}

/// Creates an SVG response with proper Content-Type header
fn svg_response(svg_content: String) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/svg+xml")],
        svg_content,
    )
        .into_response()
}

/// Creates an error response for when no icon is found
fn not_found_response(message: &str) -> Response {
    (StatusCode::NOT_FOUND, message.to_string()).into_response()
}

async fn file_icon(Query(query): Query<PathQuery>) -> Response {
    match get_icon_for_file(&query.path) {
        Some(icon_id) => match load_icon_svg(icon_id) {
            Ok(svg) => svg_response(svg),
            Err(_) => not_found_response("Icon file not found on disk"),
        },
        None => not_found_response("No icon found for file"),
    }
}

async fn folder_icon(Query(query): Query<PathQuery>) -> Response {
    match get_icon_for_folder(&query.path) {
        Some(icon_id) => match load_icon_svg(icon_id) {
            Ok(svg) => svg_response(svg),
            Err(_) => not_found_response("Icon file not found on disk"),
        },
        None => not_found_response("No icon found for folder"),
    }
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/icon/file", get(file_icon))
        .route("/icon/folder", get(folder_icon))
}
