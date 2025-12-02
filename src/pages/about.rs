use axum::{
    response::Html,
    extract::Extension,
};
use std::sync::Arc;
use tera::{Context, Tera};

pub async fn about(Extension(tera): Extension<Arc<Tera>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("title", "About RAT");
    context.insert("message", "Learn more about the Rust Archive Tool.");
    Html(tera.render("about.html", &context).unwrap())
}