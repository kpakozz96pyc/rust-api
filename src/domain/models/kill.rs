use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use serde_json::json;
use uuid::Uuid;

use crate::infra::errors::InfraError;

#[derive(Clone, Debug, PartialEq)]
pub struct Kill {
    pub id: Uuid,
    pub killer: String,
    pub killed: String,
    pub range: f32,
    pub gun: String,
    pub date: DateTime<Utc>,
    pub server: String,
}

#[derive(Debug)]
pub enum PostError {
    InternalServerError,
    NotFound(Uuid),
    InfraError(InfraError),
}

impl IntoResponse for PostError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("PostModel with id {} has not been found", id),
            ),
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };
        (
            status,
            Json(
                json!({"resource":"PostModel", "message": err_msg, "happened_at" : chrono::Utc::now() }),
            ),
        )
            .into_response()
    }
}
