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

    // include the video file at compile time
    let video_data = include_bytes!("../static/rat-demo-xz.mp4").to_vec();

    // include screenshot files at compile time
    let ss_1_data = include_bytes!("../static/ss_1.png").to_vec();
    let ss_2_data = include_bytes!("../static/ss_2.jpg").to_vec();
    // let ss_3_data = include_bytes!("../static/ss_3.jpg").to_vec();

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
        .route("/static/rat-demo-xz.mp4", get(move || async move {
            (
                axum::http::StatusCode::OK,
                [("content-type", "video/mp4")],
                video_data.clone(),
            )
        }))
        .route("/static/ss_1.png", get(move || async move {
            (
                axum::http::StatusCode::OK,
                [("content-type", "image/png")],
                ss_1_data.clone(),
            )
        }))
        .route("/static/ss_2.jpg", get(move || async move {
            (
                axum::http::StatusCode::OK,
                [("content-type", "image/jpeg")],
                ss_2_data.clone(),
            )
        }))
        // .route("/static/ss_3.jpg", get(move || async move {
        //     (
        //         axum::http::StatusCode::OK,
        //         [("content-type", "image/jpeg")],
        //         ss_3_data.clone(),
        //     )
        // }))
        .layer(Extension(tera));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(); // bind to port 3000
    println!("Listening on port 3000");
    axum::serve(listener, app).await.unwrap(); // serve the app
}