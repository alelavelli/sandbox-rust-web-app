use serde::Serialize;

use crate::UserId;

/// Authorization response for jwt token
#[derive(Serialize)]
pub struct JWTAuthResponse {
    pub token: String,
    pub token_type: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
}
