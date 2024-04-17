use axum::async_trait;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    service::db::{serialize_object_id, DatabaseDocument},
    UserId,
};

/// Struct representing user model
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id"
    )]
    pub id: Option<UserId>,
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
        let id = outcome.inserted_id.as_object_id().unwrap().to_hex();
        Ok(id)
    }
}
