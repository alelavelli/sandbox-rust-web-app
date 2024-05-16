use serde::Deserialize;

use crate::enums::Role;

/// Authorization payload for jwt token
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JWTAuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub role: Role,
}
