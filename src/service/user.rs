use crate::{model::user, UserId};

pub async fn get_user(user_id: &UserId) -> user::User {
    // make query on database
    user::User {
        id: *user_id,
        username: "Antonio".into(),
    }
}

pub async fn create_user(username: String) -> user::User {
    // make query on database
    user::User { id: 1, username }
}
