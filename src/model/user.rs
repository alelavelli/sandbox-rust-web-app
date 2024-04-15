use axum::async_trait;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::{error::AppError, service::db::DatabaseDocument, UserId};

/// Struct representing user model
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
}

#[async_trait]
impl DatabaseDocument for User {
    fn collection_name() -> &'static str {
        "User"
    }

    async fn dump(&self, db: &Database) -> Result<String, AppError> {
        let collection = db.collection::<Self>(Self::collection_name());
        let outcome = collection.insert_one(self.clone(), None).await?;
        Ok(outcome.inserted_id.to_string())
    }
}
