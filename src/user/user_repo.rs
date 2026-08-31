use std::sync::Arc;

use crate::{
    state::AppState, user::{
        dto::CreateUser, errors::UsersErrors, user_model::User
    }
};

pub async fn create_user_repo(
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

        Err(_) => Err(UsersErrors::SqlxError)
    }
}