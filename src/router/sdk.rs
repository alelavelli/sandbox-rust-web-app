use crate::{
    auth::APIKeyAuthClaim,
    dtos::{sdk_request, sdk_response, AppJson},
};

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use once_cell::sync::Lazy;
use tracing::debug;

use crate::error::AppError;

pub static SDK_ROUTER: Lazy<Router> = Lazy::new(|| {
    Router::new()
        .route("/user/:id", get(get_user))
        .route("/user", post(create_user))
});

/// Returns the user if it exists with all the information
///
/// Request parameter is extracted from the url
async fn get_user(
    api_key: APIKeyAuthClaim,
    Path(id): Path<u64>,
) -> Result<AppJson<sdk_response::User>, AppError> {
    debug!("Request made with api_key: {}", api_key.key);
    // make query on database
    let user = sdk_response::User {
        id,
        username: "Antonio".into(),
    };

    Ok(AppJson(user))
}

/// Create new user providing required attributes
async fn create_user(
    api_key: APIKeyAuthClaim,
    Json(payload): Json<sdk_request::CreateUser>,
) -> Result<AppJson<sdk_response::User>, AppError> {
    debug!("Request made with api_key: {}", api_key.key);
    // insert user in database
    let user = sdk_response::User {
        id: 1,
        username: payload.username,
    };
    Ok(AppJson(user))
}
