use serde::Deserialize;

use crate::enums::Role;

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub role: Role
}
