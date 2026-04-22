use axum::{
    Json,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{
    icon_resolver::{get_icon_for_file, get_icon_for_folder},
    rest_api::common_types::{ErrorResponse, PathQuery},
};

#[derive(Debug, Serialize)]
struct IconResponse {
    icon_id: u64,
}

pub async fn file_icon_id(Query(query): Query<PathQuery>) -> Response {
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

pub async fn folder_icon_id(Query(query): Query<PathQuery>) -> Response {
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
