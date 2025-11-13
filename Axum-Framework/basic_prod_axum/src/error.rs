use axum::{Error, Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

pub enum AppError {
    AuthError(String),
    NotFound,
    InternalServerError,
    RateLimitError,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            AppError::AuthError(msg) => (StatusCode::UNAUTHORIZED, msg.to_string()),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server Error".to_string(),
            ),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Item not found".to_string()),
            AppError::RateLimitError => {
                (StatusCode::REQUEST_TIMEOUT, "Too Many Requests".to_string())
            }
        };
        let error_response = ErrorResponse {
            message: error_message,
        };
        (status, Json(error_response)).into_response()
    }
}
