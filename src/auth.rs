use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AuthError};

/// Private keys used to encode and decode jwt tokens
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}
impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = "secret"; //std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

/// Struct containing information that will be encoded inside the jwt token
#[derive(Debug, Serialize, Deserialize)]
pub struct JWTAuthClaim {
    pub exp: usize,
    pub user_id: u64,
    pub username: String,
    pub email: String,
    pub company: String,
}

impl JWTAuthClaim {
    pub fn build_token(&self, header: &Header) -> Result<String, AuthError> {
        let token = encode(header, &self, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)?;
        Ok(token)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for JWTAuthClaim
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        tracing::debug!("Got bearer token {}", bearer.token());
        // Decode the user data
        let token_data =
            decode::<JWTAuthClaim>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|e| {
                    tracing::error!("Got error {}", e);
                    AuthError::InvalidToken
                })?;

        Ok(token_data.claims)
    }
}
