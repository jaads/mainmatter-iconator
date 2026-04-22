use crate::rest_api::{
    health::health,
    id::{file_icon_id, folder_icon_id},
    svg::{file_icon_svg, folder_icon_svg},
};
use axum::{Router, routing::get};

mod common_types;
mod health;
mod id;
mod svg;

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
