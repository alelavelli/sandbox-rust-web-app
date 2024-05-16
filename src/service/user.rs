use anyhow::anyhow;
use mongodb::bson::doc;

use crate::{
    enums::Role,
    error::{AppError, AuthError},
    model::user,
    UserId,
};

use super::db::{get_database_service, DatabaseDocument};
use base64ct::{Base64, Encoding};

pub async fn login(username: &str, password: &str) -> Result<user::User, AppError> {
    let db = &get_database_service().await.db;
    let collection = db.collection::<user::User>(user::User::collection_name());
    let hashed_password = hash_password(password);
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
pub async fn create_user(
    username: String,
    password: String,
    role: Role,
) -> Result<String, AppError> {
    let user_model = user::User {
        id: None,
        username,
        password_hash: hash_password(&password),
        api_key: None,
        role,
    };
    let db_service = get_database_service().await;
    user_model.dump(&db_service.db).await
}

fn hash_password(password: &str) -> String {
    Base64::encode_string(password.as_bytes())
}

#[cfg(test)]
mod tests {
    use crate::{
        enums::Role,
        model::user,
        service::{
            db::{get_database_service, DatabaseDocument},
            user::{create_user, hash_password},
        },
    };

    use super::login;

    #[tokio::test]
    async fn create_user_test() {
        let username = "John".into();
        let password = "Smith".into();
        let role = Role::Admin;

        let created_user_result = create_user(username, password, role).await;
        assert!(created_user_result.is_ok());
        let drop_result = get_database_service().await.db.drop(None).await;
        assert!(drop_result.is_ok())
    }

    #[tokio::test]
    async fn login_test() {
        let username = "John";
        let password = "Smith";
        let role = Role::Admin;

        // No users
        let result = login(username, password).await;
        assert!(result.is_err());

        // Add users and retrieve them
        let user_id_result = user::User {
            id: None,
            username: username.into(),
            password_hash: hash_password(password),
            api_key: None,
            role,
        }
        .dump(&get_database_service().await.db)
        .await;
        assert!(user_id_result.is_ok());

        // Remake the query
        let result = login(username, password).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(username, user.username);
        assert_eq!(role, user.role);
        let drop_result = get_database_service().await.db.drop(None).await;
        assert!(drop_result.is_ok());
    }
}
