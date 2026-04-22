use axum::http::header;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::file_loader::load_icon_svg;
use crate::icon_resolver::{get_icon_for_file, get_icon_for_folder};
use crate::rest_api::common_types::PathQuery;

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

pub async fn file_icon_svg(Query(query): Query<PathQuery>) -> Response {
    match get_icon_for_file(&query.path) {
        Some(icon_id) => match load_icon_svg(icon_id) {
            Ok(svg) => svg_response(svg),
            Err(_) => not_found_response("Icon file not found on disk"),
        },
        None => not_found_response("No icon found for file"),
    }
}

pub async fn folder_icon_svg(Query(query): Query<PathQuery>) -> Response {
    match get_icon_for_folder(&query.path) {
        Some(icon_id) => match load_icon_svg(icon_id) {
            Ok(svg) => svg_response(svg),
            Err(_) => not_found_response("Icon file not found on disk"),
        },
        None => not_found_response("No icon found for folder"),
    }
}
