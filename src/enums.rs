use serde::{Deserialize, Serialize};

/// Enumeration with roles assigned to Users
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Role {
    /// Basic user
    User,
    /// Admin user
    Admin,
}
