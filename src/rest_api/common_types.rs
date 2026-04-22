use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PathQuery {
    pub(crate) path: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub(crate) error: String,
}
