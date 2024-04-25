use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::dtos::AppJson;

/// AppError enumeration of different error typologies that the application
/// can return to clients.
///
/// It implements the trait `IntoResponse` to translate the error into a response
/// composed of status and message.
///
/// Moreover, it implements several `From<T>` trait to automatically translate
/// internal errors to AppError using `?`
#[derive(Debug)]
pub enum AppError {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
    // Internal error
    InternalServerError(anyhow::Error),
    // Authorization error
    AuthorizationError(AuthError),
    // Entity does not exist
    DoesNotExist(anyhow::Error),
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }
        // Define StatusCode and message for every enum variant
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
            AppError::DoesNotExist(_) => (StatusCode::NOT_FOUND, "Entity not found".into()),
        };
        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

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

impl From<mongodb::error::Error> for AppError {
    fn from(value: mongodb::error::Error) -> Self {
        Self::InternalServerError(anyhow::Error::new(value))
    }
}

/// AuthError is an internal error used by authentication modules to explain why
/// authentication is failed.
/// They are translated to `AppError` when exposed to the client
#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    InvalidApiKey,
}

impl AuthError {
    fn to_status_message(&self) -> (StatusCode, String) {
        let (status, message) = match self {
            AuthError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, "Wrong credentials".to_string())
            }
            AuthError::InvalidApiKey => (StatusCode::UNAUTHORIZED, "Wrong credentials".to_string()),

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
