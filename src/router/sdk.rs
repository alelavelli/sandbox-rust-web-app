use crate::{
    auth::APIKeyAuthClaim,
    dtos::{sdk_request, sdk_response, AppJson},
    UserId,
};

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use once_cell::sync::Lazy;
use tracing::debug;

use crate::error::AppError;
use crate::facade::sdk as facade;

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
    Path(id): Path<UserId>,
) -> Result<AppJson<sdk_response::User>, AppError> {
    let user = facade::get_user(api_key, id).await?;
    Ok(AppJson(user))
}

/// Create new user providing required attributes
async fn create_user(
    api_key: APIKeyAuthClaim,
    Json(payload): Json<sdk_request::CreateUser>,
) -> Result<AppJson<String>, AppError> {
    let user = facade::create_user(api_key, payload).await?;
    Ok(AppJson(user))
}
