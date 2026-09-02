use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

use super::user_model::UserRole;

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct PublicUser {
    pub name: String,
    pub email: String,
    pub description: Option<String>,
    pub role_id: i32,
    pub uuid: Uuid
}