mod api;
mod config;
mod errors;
mod models;
mod services;
mod utils;

use std::net::SocketAddr;

use axum::Router;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::Method;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Create necessary directories
    for dir in ["uploads", "latex", "pdf"] {
        tokio::fs::create_dir_all(dir)
            .await
            .expect(&format!("Failed to create {} directory", dir));
    }

    // Load environment variables
    dotenv::dotenv().ok();

    let app = Router::new()
        .merge(api::routes())
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://localhost:5173".parse().unwrap(),
                    "http://localhost:3000".parse().unwrap(),
                    "https://noteforge-nu.vercel.app".parse().unwrap(),
                    "https://noteforge-2oepmnj85-g4titans-projects.vercel.app".parse().unwrap(),
                ])
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers([CONTENT_TYPE, AUTHORIZATION])
                .allow_credentials(true),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
