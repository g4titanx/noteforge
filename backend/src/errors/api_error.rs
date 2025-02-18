use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Authentication failed")]
    AuthenticationError,

    #[error("Authorization failed")]
    AuthorizationError,

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("File error: {0}")]
    FileError(String),

    #[error("Claude API error: {0}")]
    ClaudeError(String),

    #[error("LaTeX conversion error: {0}")]
    LaTeXError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal server error")]
    InternalServerError(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::AuthenticationError => (StatusCode::UNAUTHORIZED, self.to_string()),
            ApiError::AuthorizationError => (StatusCode::FORBIDDEN, self.to_string()),
            ApiError::NotFound(ref message) => (StatusCode::NOT_FOUND, message.to_owned()),
            ApiError::ValidationError(ref message) => (StatusCode::BAD_REQUEST, message.to_owned()),
            ApiError::FileError(ref message) => (StatusCode::BAD_REQUEST, message.to_owned()),
            ApiError::ClaudeError(ref message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.to_owned())
            }
            ApiError::LaTeXError(ref message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.to_owned())
            }
            ApiError::DatabaseError(ref message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.to_owned())
            }
            ApiError::InternalServerError(ref e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "code": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}
