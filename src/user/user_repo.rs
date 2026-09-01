use std::sync::Arc;

use crate::{
    state::AppState, user::{
        dto::PublicUser,
        errors::UsersErrors,
        user_model::User
    }
};

pub async fn create_user(
    state: &Arc<AppState>,
    user: User
) -> Result<(), UsersErrors> {

    let result = sqlx::query(
        r#"
        INSERT INTO users
            (name, email, password_hash, uuid)
        VALUES
            ($1, $2, $3, $4)
        "#
    )
    .bind(&user.name)
    .bind(&user.email)
    .bind(&user.password)
    .bind(&user.uuid)
    .execute(&state.postgres)
    .await;

    match result {
        Ok(_) => Ok(()),

        Err(sqlx::Error::Database(db_error)) if db_error.is_check_violation() => {  
            match db_error.constraint() {
                Some("users_name_key") => {
                    Err(UsersErrors::NameAlreadyRegistered)
                }
                Some("users_email_key") => {
                    Err(UsersErrors::EmailAlreadyRegistered)
                }
                _ => {
                    Err(UsersErrors::SqlxError)
                }
            }
        },

        Err(error) => {
            println!("{:?}", error);
            Err(UsersErrors::SqlxError)
        }
    }
}

pub async fn get_users(
    state: &Arc<AppState>
) ->  Result<Vec<PublicUser>, UsersErrors> {
    let users = sqlx::query_as::<_, PublicUser>(
        r#"
        SELECT 
            name,
            email,
            description,
            role_id,
            uuid
        FROM users
        "#
    ).fetch_all(&state.postgres)
    .await;

    match users {
        Ok(users) => Ok(users),
        Err(error) => {
            println!("{:?}", error);
            Err(UsersErrors::SqlxError)
        }
    }
}