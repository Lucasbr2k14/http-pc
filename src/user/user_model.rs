use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
enum UserRole {
    ADMIN,
    REGULAR,
    USER
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub role: Option<UserRole>,
    pub uuid: Uuid
}

impl User {
    pub fn new(
        name: String, 
        email:String,
        password: Option<String>,
    ) -> Self {

        Self { 
            name: name, 
            email: email, 
            password: password, 
            role: None, 
            uuid: Uuid::now_v7()
        }
    
    }
}
