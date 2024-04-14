//! Environment service use to build and store all the application environment variables.
//!
//! This struct loads the application variables from the environment or other secret manager endpoints
//! providing them to other services.
//! It represents the true and unique source of application variables

use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

/// ENVIRONMENT struct containing application variables
pub static ENVIRONMENT: Lazy<EnvironmentVariables> = Lazy::new(EnvironmentVariables::new);

/// Struct containing application environment variables that is initialized from
/// environment or accessing external services
pub struct EnvironmentVariables {
    pub authentication: AuthenticationVariables,
}

impl EnvironmentVariables {
    /// Create new instance of this struct by invoking the different builds functions
    fn new() -> Self {
        EnvironmentVariables {
            authentication: Self::build_authentication(),
        }
    }

    /// Build authentication variables
    ///
    /// Environment variable `JWT_SECRET` is used to create JWT encoding and decoding keys
    /// therefore, it is mandatory.
    fn build_authentication() -> AuthenticationVariables {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        AuthenticationVariables {
            jwt_encoding: EncodingKey::from_secret(secret.as_bytes()),
            jwt_decoding: DecodingKey::from_secret(secret.as_bytes()),
        }
    }
}

/// Struct containing variables for authentication
///
/// It contains two keys used to encode and decode jwt tokens for web application
pub struct AuthenticationVariables {
    pub jwt_encoding: EncodingKey,
    pub jwt_decoding: DecodingKey,
}
