use std::sync::Arc;
use crate::{security::password, user::errors::UsersErrors};

use sqlx::Error;


use crate::{
    state::AppState, 
    user::{
        dto::CreateUser, 
        user_model::User,
        user_repo::{ create_user_repo },
    }
};

pub async fn create_user (
    state: &Arc<AppState>,
    create_user: &CreateUser
) -> Result<(), UsersErrors> {
    
    let password_hash = password::hash_pass(
        &create_user.password
    ).expect("Error to create password hash."); 
    
    let user = User::new(
        create_user.name.clone(), 
        create_user.email.clone(),
        Some(password_hash.clone())
    );

    create_user_repo(state, user).await
}   