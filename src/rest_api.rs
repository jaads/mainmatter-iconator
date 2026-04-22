use axum::{Json, Router, extract::Query, http::StatusCode, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};

use crate::icon_resolver::{get_icon_for_file, get_icon_for_folder};

#[derive(Debug, Deserialize)]
struct PathQuery {
    path: String,
}

#[derive(Debug, Serialize)]
struct IconResponse {
    icon_id: u64,
    path: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    path: String,
}

async fn file_icon(Query(query): Query<PathQuery>) -> impl IntoResponse {
    match get_icon_for_file(&query.path) {
        Some(icon_id) => (
            StatusCode::OK,
            Json(IconResponse {
                icon_id,
                path: query.path,
            }),
        )
            .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "No icon found for file".to_string(),
                path: query.path,
            }),
        )
            .into_response(),
    }
}

async fn folder_icon(Query(query): Query<PathQuery>) -> impl IntoResponse {
    match get_icon_for_folder(&query.path) {
        Some(icon_id) => (
            StatusCode::OK,
            Json(IconResponse {
                icon_id,
                path: query.path,
            }),
        )
            .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "No icon found for folder".to_string(),
                path: query.path,
            }),
        )
            .into_response(),
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
