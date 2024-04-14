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
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AuthError};

/// Private keys used to encode and decode jwt tokens
struct JWTKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}
impl JWTKeys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: Lazy<JWTKeys> = Lazy::new(|| {
    let secret = "secret"; //std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    JWTKeys::new(secret.as_bytes())
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

/// Struct containing api key authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct APIKeyAuthClaim {
    pub key: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for APIKeyAuthClaim
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(apikey)) = parts
            .extract::<TypedHeader<Authorization<ApiKey>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        // Make check of this api key is valid
        // ...

        let auth_data = APIKeyAuthClaim {
            key: apikey.key().into(),
        };

        Ok(auth_data)
    }
}

#[derive(Clone, PartialEq, Debug)]
/// Token holder for Bearer Authentication, most often seen with oauth
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
