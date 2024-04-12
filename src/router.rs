use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::Header;

use crate::{
    auth::JWTAuthClaim,
    dtos::{AppJson, CreateUser, GetUser, JWTAuthPayload, JWTAuthResponse, User},
    error::{AppError, AuthError},
};

/// Defines the router user that is imported by the main application Router
pub fn get_user_router() -> Router {
    Router::new()
        .route("/login", post(authorize))
        .route("/protected", get(protected_get_user))
        .route("/", post(create_user))
        .route("/", get(handler))
        .route("/all", get(get_user))
        .route("/:id", get(get_user_by_id))
}

/// Authorize a user with username and password providing jwt token
async fn authorize(
    Json(payload): Json<JWTAuthPayload>,
) -> Result<AppJson<JWTAuthResponse>, AppError> {
    // check username and password on database
    if payload.username.is_empty() | payload.password.is_empty() {
        Err(AuthError::MissingCredentials)?
    } else if payload.username.cmp(&"sonoio".into()).is_ne() {
        Err(AuthError::WrongCredentials)?
    } else {
        let claims = JWTAuthClaim {
            exp: 2000000000,
            user_id: 3,
            username: "ciao".into(),
            email: "ciao".into(),
            company: "ciao".into(),
        };
        let token = claims.build_token(&Header::default())?;
        Ok(AppJson(JWTAuthResponse {
            token,
            token_type: "Bearer".into(),
        }))
    }
}

async fn protected_get_user(
    jwt_claim: JWTAuthClaim,
    Json(payload): Json<GetUser>,
) -> Result<AppJson<User>, AppError> {
    tracing::debug!("Logged with user id {}", jwt_claim.user_id);
    let user = User {
        id: jwt_claim.user_id,
        username: payload.username,
    };

    Ok(AppJson(user))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> Result<AppJson<User>, AppError> {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    Ok(AppJson(user))
}

async fn handler() -> Result<AppJson<()>, AppError> {
    try_thing()?;
    Ok(AppJson(()))
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}

async fn get_user(Json(payload): Json<GetUser>) -> Result<AppJson<User>, AppError> {
    let user = make_db_query(payload.username)?;
    Ok(AppJson(user))
}

// Mock function that returns an error
fn make_db_query(_username: String) -> Result<User, anyhow::Error> {
    anyhow::bail!("internal connection failed")
}

async fn get_user_by_id(Path(id): Path<u64>) -> Result<AppJson<User>, AppError> {
    let user = User {
        id,
        username: "my user".into(),
    };

    Ok(AppJson(user))
}
