use crate::{
    auth::JWTAuthClaim,
    dtos::{web_app_request, web_app_response, AppJson},
    UserId,
};

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use once_cell::sync::Lazy;

use crate::error::AppError;
use crate::facade::web_app as facade;

pub static WEB_APP_ROUTER: Lazy<Router> = Lazy::new(|| {
    Router::new()
        .route("/login", post(authorize))
        .route("/user/:id", get(get_user))
        .route("/user", post(create_user))
});

/// Authorize a user with username and password providing jwt token
async fn authorize(
    Json(payload): Json<web_app_request::JWTAuthPayload>,
) -> Result<AppJson<web_app_response::JWTAuthResponse>, AppError> {
    facade::authenticate_user(&payload.username, &payload.password)
        .await
        .map(AppJson)
}

/// Returns the user if it exists with all the information
///
/// Request parameter is extracted from the url
async fn get_user(
    jwt_claim: JWTAuthClaim,
    Path(id): Path<UserId>,
) -> Result<AppJson<web_app_response::User>, AppError> {
    let user = facade::get_user(jwt_claim, id).await?;
    Ok(AppJson(user))
}

/// Create new user providing required attributes
async fn create_user(
    jwt_claim: JWTAuthClaim,
    Json(payload): Json<web_app_request::CreateUser>,
) -> Result<AppJson<String>, AppError> {
    let user = facade::create_user(jwt_claim, payload).await?;
    Ok(AppJson(user))
}
