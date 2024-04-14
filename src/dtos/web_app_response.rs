use serde::Serialize;

/// Authorization response for jwt token
#[derive(Serialize)]
pub struct JWTAuthResponse {
    pub token: String,
    pub token_type: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}
