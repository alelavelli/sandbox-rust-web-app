use tracing::debug;

use crate::{
    auth::AuthInfo,
    dtos::{sdk_request, sdk_response},
    UserId,
};

pub async fn get_user(auth_info: impl AuthInfo, user_id: UserId) -> sdk_response::User {
    // access control over
    debug!(
        "Making access control for auth_info with user {}",
        auth_info.user_id()
    );
    // make query on database
    sdk_response::User {
        id: user_id,
        username: "Antonio".into(),
    }
}

pub async fn create_user(
    auth_info: impl AuthInfo,
    payload: sdk_request::CreateUser,
) -> sdk_response::User {
    // access control over
    debug!(
        "Making access control for auth_info with user {}",
        auth_info.user_id()
    );
    // make query on database
    sdk_response::User {
        id: 1,
        username: payload.username,
    }
}