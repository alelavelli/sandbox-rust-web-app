use axum::{
    extract::Path, routing::{get, post}, Json, Router
};

use crate::{
    dtos::{AppJson, CreateUser, GetUser, User},
    error::AppError,
};

/// Defines the router user that is imported by the main application Router
pub fn get_user_router() -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(handler))
        .route("/all", get(get_user))
        .route("/:id", get(get_user_by_id))
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
        id: id,
        username: "my user".into(),
    };

    Ok(AppJson(user))
}
