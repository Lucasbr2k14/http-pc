use std::{f32::consts::E, sync::Arc};

use askama::Template;
use axum::{
    Form,
    extract::State, 
    http::StatusCode, 
    response::{ 
        Html,
        IntoResponse,
        Json,
        Redirect,
        Response
    }
};

use crate::{
    state::AppState, user::{dto::PublicUser, user_model::User}
};

use super::{
    dto,
    user_services,
    errors::UsersErrors
};


/*
TODO: Criar user
TODO: Deletar user
TODO: Get user
TODO: UPDATE [Senha, Descição]
*/

#[derive(Template)]
#[template(path = "register.html.jinja")]
struct RegisterTemplate {
    error: Option<String>
}

pub async fn register_web() -> Html<String>{
    let t = RegisterTemplate { error: None };
    Html(t.render().unwrap())
}

pub async fn create_user (
    State(state): State<Arc<AppState>>,
    Form(form): Form<dto::CreateUser>
) -> Response {

    let users_register = user_services::create_user(&state, &form).await;

    match users_register {
   
        Ok(_) => {
            Redirect::to("/login").into_response()
        },
    
        Err( UsersErrors::EmailAlreadyRegistered ) => {
            let template_error = RegisterTemplate {
                error: Some("Email já registrado".to_string())
            };
            (
                StatusCode::CONFLICT, 
                Html(template_error.render().unwrap())
            ).into_response()
        }

        Err( UsersErrors::NameAlreadyRegistered ) => {
            let template_error = RegisterTemplate {
                error: Some("Nome já registrado".to_string())
            };
            (
                StatusCode::CONFLICT,
                Html(template_error.render().unwrap())
            ).into_response()
        }

        Err(_) => {
            let template_error = RegisterTemplate { 
                error: Some("Erro interno".to_string()) 
            };
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(template_error.render().unwrap())
            ).into_response()
        }

    }
}

#[derive(Template)]
#[template(path = "users.html.jinja")]
struct UsersTemplate {
    users: Vec<PublicUser>
}

pub async fn users (
    State(state): State<Arc<AppState>>,
) -> Response {
    
    match user_services::get_all_users(&state).await {
        
        Ok(users) => { 
            let template = UsersTemplate{
                users:users
            };
            (
                StatusCode::OK,
                Html(template.render().unwrap()) 
            ).into_response()
        }
        
        Err(_) => { 
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Internal error"
            ).into_response()
        }
    }
}

pub async fn get_user() {}
pub async fn delete_user() {}
pub async fn update_user() {}