use serde::Serialize;

use crate::UserId;

#[derive(Serialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
}
