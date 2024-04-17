use anyhow::anyhow;
use mongodb::bson::doc;

use crate::{error::AppError, model::user, UserId};

use super::db::{get_database_service, DatabaseDocument};

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
pub async fn create_user(username: String) -> Result<String, AppError> {
    let user_model = user::User { id: None, username };
    let db_service = get_database_service().await;
    user_model.dump(&db_service.db).await
}
