mod convert;
mod health;
mod upload;

use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .merge(health::routes())
        .route("/upload", post(upload::handle_upload))
        .route("/convert/:file_id", get(convert::convert_to_text))
}
