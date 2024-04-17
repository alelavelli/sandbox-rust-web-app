use anyhow::anyhow;
use mongodb::bson::doc;

use crate::{
    error::{AppError, AuthError},
    model::user,
    UserId,
};

use super::db::{get_database_service, DatabaseDocument};
use base64ct::{Base64, Encoding};

pub async fn login(username: &str, password: &str) -> Result<user::User, AppError> {
    let db = &get_database_service().await.db;
    let collection = db.collection::<user::User>(user::User::collection_name());
    let hashed_password = hash_password(&password);
    let filter = doc! {
        "username": username,
        "password_hash": hashed_password
    };
    let query_result = collection.find_one(filter, None).await?;
    if let Some(user_document) = query_result {
        Ok(user_document)
    } else {
        Err(AuthError::WrongCredentials)?
    }
}

pub async fn get_user(user_id: &UserId) -> Result<user::User, AppError> {
    let db = &get_database_service().await.db;
    let collection = db.collection::<user::User>(user::User::collection_name());
    let filter = doc! { "_id": user_id };
    let query_result = collection.find_one(filter, None).await?;
    if let Some(user_document) = query_result {
        Ok(user_document)
    } else {
        Err(AppError::DoesNotExist(anyhow!(
            "User with id {user_id} does not exist"
        )))
    }
}

/// Create new user in database and returns it identifier
pub async fn create_user(username: String, password: String) -> Result<String, AppError> {
    let user_model = user::User {
        id: None,
        username,
        password_hash: hash_password(&password),
        api_key: None,
    };
    let db_service = get_database_service().await;
    user_model.dump(&db_service.db).await
}

fn hash_password(password: &str) -> String {
    Base64::encode_string(password.as_bytes())
}
