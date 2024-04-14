use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderValue},
    RequestPartsExt,
};
use axum_extra::{
    headers::{
        authorization::{Bearer, Credentials},
        Authorization,
    },
    TypedHeader,
};
use jsonwebtoken::{decode, encode, Header, Validation};

use serde::{Deserialize, Serialize};

use crate::{
    error::{AppError, AuthError},
    service::environment::ENVIRONMENT,
    UserId,
};

/// Trait for auth info objects that need to return specific information
pub trait AuthInfo {
    fn user_id(&self) -> &UserId;
}

/// Struct containing information that will be encoded inside the jwt token
#[derive(Debug, Serialize, Deserialize)]
pub struct JWTAuthClaim {
    pub exp: usize,
    pub user_id: UserId,
    pub username: String,
    pub email: String,
    pub company: String,
}

impl JWTAuthClaim {
    pub fn build_token(&self, header: &Header) -> Result<String, AuthError> {
        let token = encode(header, &self, &ENVIRONMENT.authentication.jwt_encoding)
            .map_err(|_| AuthError::TokenCreation)?;
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
        let token_data = decode::<JWTAuthClaim>(
            bearer.token(),
            &ENVIRONMENT.authentication.jwt_decoding,
            &Validation::default(),
        )
        .map_err(|e| {
            tracing::error!("Got error {}", e);
            AuthError::InvalidToken
        })?;

        Ok(token_data.claims)
    }
}

#[async_trait]
impl AuthInfo for JWTAuthClaim {
    fn user_id(&self) -> &UserId {
        &self.user_id
    }
}

/// Struct containing api key authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct APIKeyAuthClaim {
    pub key: String,
    pub user_id: UserId,
}

#[async_trait]
impl<S> FromRequestParts<S> for APIKeyAuthClaim
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(api_key)) = parts
            .extract::<TypedHeader<Authorization<ApiKey>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        // Check if this api key is valid
        // ...

        let auth_data = APIKeyAuthClaim {
            user_id: 1,
            key: api_key.key().into(),
        };

        Ok(auth_data)
    }
}

#[async_trait]
impl AuthInfo for APIKeyAuthClaim {
    fn user_id(&self) -> &UserId {
        &self.user_id
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ApiKey(String);

impl ApiKey {
    /// View the token part as a `&str`.
    pub fn key(&self) -> &str {
        self.0.as_str()["x-api-key ".len()..].trim_start()
    }
}

impl Credentials for ApiKey {
    const SCHEME: &'static str = "x-api-key";

    fn decode(value: &HeaderValue) -> Option<Self> {
        debug_assert!(
            value.as_bytes()[..Self::SCHEME.len()].eq_ignore_ascii_case(Self::SCHEME.as_bytes()),
            "HeaderValue to decode should start with \"x-api-key ..\", received = {:?}",
            value,
        );

        value.to_str().ok().map(|s| ApiKey(s.to_string()))
    }

    fn encode(&self) -> HeaderValue {
        HeaderValue::from_str(&self.0).unwrap()
    }
}
