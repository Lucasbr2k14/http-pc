use std::sync::Arc;
use redis::AsyncCommands;
use uuid::Uuid;

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

pub async fn get_user(
    user_id: Uuid,
    state: &Arc<AppState>
) ->  Result<PublicUser, UsersErrors> {
    // TODO: Voltar aqui quando o admin for implementado.
    // Para mostrar informações mais restritas

    let mut redis = state.redis.clone();
    let key = format!("users:{}", user_id.to_string());
    
    let cache: Option<String> = redis.get(&key)
    .await
    .map_err(|_| UsersErrors::RedisError)?;

    if let Some(json) = cache {
        let user: PublicUser = serde_json::from_str(&json)
        .map_err(|_| UsersErrors::SerializationError)?;
        return Ok(user);
    } 
    
    match user_repo::get_user(user_id, &state).await {
        Ok(user) => {
            let user_str = serde_json::to_string(&user)
            .map_err(|_| UsersErrors::SerializationError)?;
            let _:() = redis.set_ex(&key, user_str, 60 * 3)
            .await
            .map_err(|_| UsersErrors::RedisError)?;
            Ok(user)
        },

        Err(e) => Err(e)
    } 
}