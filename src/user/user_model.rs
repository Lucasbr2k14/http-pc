use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
enum UserRole {
    Admin,
    Regular
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    role: UserRole,
}

impl User {
    pub fn new(name: String, user_role: UserRole) -> User {
        User {
            name:name, 
            role: user_role
        }
    }
}
