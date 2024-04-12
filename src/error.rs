use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::dtos::AppJson;

// Make our own error that wraps `anyhow::Error`.
pub enum AppError {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
    // Internal error
    InternalServerError(anyhow::Error),
    // Authorization error
    AuthorizationError(AuthError),
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }
        let (status, message) = match self {
            AppError::JsonRejection(rejection) => {
                // This error is caused by bad user input so don't log it
                (rejection.status(), rejection.body_text())
            }
            AppError::InternalServerError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".into(),
            ),
            AppError::AuthorizationError(auth_error) => auth_error.to_status_message(),
        };
        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl From<JsonRejection> for AppError {
    fn from(value: JsonRejection) -> Self {
        Self::JsonRejection(value)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self::InternalServerError(value)
    }
}

impl From<AuthError> for AppError {
    fn from(value: AuthError) -> Self {
        Self::AuthorizationError(value)
    }
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl AuthError {
    fn to_status_message(&self) -> (StatusCode, String) {
        let (status, message) = match self {
            AuthError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, "Wrong credentials".to_string())
            }
            AuthError::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Missing credentials".to_string())
            }
            AuthError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Token creation error".to_string(),
            ),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token".to_string()),
        };
        (status, message)
    }
}
