// incllude pages
mod pages;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // define the different routes
    let app = Router::new().route("/", get(pages::homepage::homepage));
    let app = app.route("/about", get(pages::about::about));
    let app = app.route("/install", get(pages::install::install));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(); // bind to port 3000
    axum::serve(listener, app).await.unwrap(); // serve the app
    
}