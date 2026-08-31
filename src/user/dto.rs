use serde::Deserialize;
use uuid::Uuid;
use sqlx::FromRow;


#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String
}
#[derive(Debug, Deserialize, FromRow)]
pub struct PublicUser {
    pub name: String,
    pub email: String,
    pub description: String,
    pub uuid: Uuid
}