use std::sync::Arc;
use crate::{
    security::password, 
    user::errors::UsersErrors
};


use crate::{
    state::AppState, 
    user::{
        dto:: { CreateUser, PublicUser}, 
        user_model::User,
        user_repo
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

    user_repo::create_user(state, user).await
}

pub async fn get_all_users( 
    state: &Arc<AppState> 
) -> Result<Vec<PublicUser>, UsersErrors>{
    user_repo::get_users(state).await 
}