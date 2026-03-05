use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Not found")]
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Paste not found"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
