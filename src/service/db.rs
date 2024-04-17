use axum::async_trait;
use mongodb::{options::ClientOptions, Client, Database};

use crate::{error::AppError, service::environment::ENVIRONMENT};

use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::Serializer;

use tokio::sync::OnceCell;

// differently from other global variables, database initialization requires async futures
// therefore, we use tokio OnceCell and an async coroutine to initialize or get it.

static DATABASE: OnceCell<DatabaseService> = OnceCell::const_new();

pub async fn get_database_service() -> &'static DatabaseService {
    DATABASE
        .get_or_init(|| async {
            DatabaseService::new()
                .await
                .expect("Error in database initialization")
        })
        .await
}

/// Database service struct that contain access to the database
pub struct DatabaseService {
    pub db: Database,
}

impl DatabaseService {
    async fn new() -> Result<DatabaseService, AppError> {
        let client_options = ClientOptions::parse(&ENVIRONMENT.database.connection_string).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&ENVIRONMENT.database.db_name);
        Ok(DatabaseService { db })
    }
}

#[async_trait]
pub trait DatabaseDocument {
    fn collection_name() -> &'static str;
    async fn dump(&self, db: &Database) -> Result<String, AppError>;
}

pub fn serialize_object_id<S>(
    object_id: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match object_id {
        Some(ref object_id) => serialize_object_id_as_hex_string(object_id, serializer),
        None => serializer.serialize_none(),
    }
}
