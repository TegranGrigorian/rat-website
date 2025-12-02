// incllude pages
mod pages;

use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use std::sync::Arc;
use tera::Tera;

#[tokio::main]
async fn main() {
    // initialize Tera templates once and share via Arc
    let tera = Tera::new("src/templates/**/*.html").expect("failed to load templates");
    let tera = Arc::new(tera);

    // small static route for the stylesheet (simple and robust)
    // use `include_str!` so the CSS is embedded at compile time
    let styles = include_str!("../static/styles.css").to_string();

    // define the routes and attach the shared Tera instance as an Extension
    let app = Router::new()
        .route("/", get(pages::homepage::homepage))
        .route("/about", get(pages::about::about))
        .route("/install", get(pages::install::install))
        .route("/static/styles.css", get(move || async move {
            (
                axum::http::StatusCode::OK,
                [("content-type", "text/css")],
                styles.clone(),
            )
        }))
        .layer(Extension(tera));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(); // bind to port 3000
    axum::serve(listener, app).await.unwrap(); // serve the app
}