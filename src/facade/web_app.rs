use jsonwebtoken::Header;

use tracing::debug;

use crate::{
    auth::{AuthInfo, JWTAuthClaim},
    dtos::{web_app_request, web_app_response},
    error::AppError,
    service::user,
    UserId,
};

pub async fn authenticate_user(
    username: &str,
    password: &str,
) -> Result<web_app_response::JWTAuthResponse, AppError> {

    let user_model = user::login(username, password).await?;

    let claims = JWTAuthClaim {
        exp: 2000000000,
        user_id: user_model.id.expect("User id must be not missing"),
        username: user_model.username,
    };
    let token = claims.build_token(&Header::default())?;
    
    Ok(web_app_response::JWTAuthResponse {
        token,
        token_type: "Bearer".into(),
    })
}

pub async fn get_user(
    auth_info: impl AuthInfo,
    user_id: UserId,
) -> Result<web_app_response::User, AppError> {
    // access control over auth info
    debug!(
        "Making access control for auth_info with user {}",
        auth_info.user_id()
    );

    let user_model = user::get_user(&user_id).await?;
    Ok(web_app_response::User {
        id: user_model
            .id
            .expect("field user_id should exist since the model comes from a db query"),
        username: user_model.username,
    })
}

pub async fn create_user(
    auth_info: impl AuthInfo,
    payload: web_app_request::CreateUser,
) -> Result<String, AppError> {
    // access control over auth info
    debug!(
        "Making access control for auth_info with user {}",
        auth_info.user_id()
    );

    user::create_user(payload.username, payload.password).await
}
