use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub fn routes() -> Router {
    Router::new().route("/health", get(health_check))
}
