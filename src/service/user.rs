use crate::{error::AppError, model::user, UserId};

use super::db::{get_database_service, DatabaseDocument};

pub async fn get_user(user_id: &UserId) -> user::User {
    // make query on database
    user::User {
        id: *user_id,
        username: "Antonio".into(),
    }
}

/// Create new user in database and returns it identifier
pub async fn create_user(username: String) -> Result<String, AppError> {
    let user_model = user::User { id: 1, username };
    let db_service = get_database_service().await;
    user_model.dump(&db_service.db).await
}
