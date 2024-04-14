use serde::Deserialize;

/// Authorization payload for jwt token
#[derive(Deserialize)]
pub struct JWTAuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}
