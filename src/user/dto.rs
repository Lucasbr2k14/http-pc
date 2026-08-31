use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String
}