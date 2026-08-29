use askama::Template; 

use super::super::AppState;

use axum::{
    response::Html
    extract::State
};

#[derive(Template)]
#[template(path = "index.html.jinja")]
struct Index;

pub async fn root(

) -> Html<String> {
    let template = Index.render().unwrap();
    Html(template)
}