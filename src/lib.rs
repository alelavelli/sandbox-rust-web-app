use mongodb::bson::oid::ObjectId;

mod auth;
mod dtos;
mod error;
mod facade;
pub mod middleware;
mod model;
mod enums;
pub mod router;
pub mod service;

type UserId = ObjectId;
