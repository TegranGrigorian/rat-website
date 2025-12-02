use axum::{
    response::Html,
};
use tera::{Context, Tera};


pub async fn homepage() -> Html<String> {
    let tera = Tera::new("src/templates/*.html").unwrap(); // look at tempaltes in src/templates
    let mut context = Context::new();
    context.insert("title", "Rust Website");
    context.insert("message", "Welcome to My Rust Website!");
    Html(tera.render("homepage.html", &context).unwrap()) // grab out homepage.html
}