use axum::{
    response::Html,
    extract::Extension,
};
use std::sync::Arc;
use tera::{Context, Tera};

pub async fn install(Extension(tera): Extension<Arc<Tera>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("title", "Install RAT");
    context.insert("message", "Installation instructions and packages.");
    Html(tera.render("install.html", &context).unwrap())
}