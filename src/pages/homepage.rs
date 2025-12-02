use axum::{
    response::Html,
    extract::Extension,
};
use std::sync::Arc;
use tera::{Context, Tera};

pub async fn homepage(Extension(tera): Extension<Arc<Tera>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("title", "RAT — Rust Archive Tool");
    context.insert("message", "A tiny, friendly tar.gz tool written in Rust.");
    Html(tera.render("homepage.html", &context).unwrap())
}