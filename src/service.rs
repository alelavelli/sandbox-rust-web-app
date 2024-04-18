//! Service module that contain application services
//!
//! Application services contain core business logic of out Application,
//! they are used by Facades to perform operations, read and store entities on
//! database.
//!
//! Services split into two main categories:
//!     - support services are used by the whole application (other services included)
//!       and provide utilities. They are environment and database service;
//!     - application services are part of the actual application and may use support services.
//!

pub mod db;
pub mod environment;
pub mod user;
