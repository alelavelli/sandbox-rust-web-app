use tracing::debug;

use crate::{
    auth::AuthInfo,
    dtos::{sdk_request, sdk_response},
    error::AppError,
    service::user,
    UserId,
};

pub async fn get_user(
    auth_info: impl AuthInfo,
    user_id: UserId,
) -> Result<sdk_response::User, AppError> {
    // access control over auth info
    debug!(
        "Making access control for auth_info with user {}",
        auth_info.user_id()
    );

    let user_model = user::get_user(&user_id).await?;
    Ok(sdk_response::User {
        id: user_model
            .id
            .expect("field user_id should exist since the model comes from a db query"),
        username: user_model.username,
    })
}

pub async fn create_user(
    auth_info: impl AuthInfo,
    payload: sdk_request::CreateUser,
) -> Result<String, AppError> {
    // access control over auth info
    debug!(
        "Making access control for auth_info with user {}",
        auth_info.user_id()
    );

    user::create_user(payload.username, payload.password).await
}
