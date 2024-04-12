use crate::controllers::index;
use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new().route("/", get(index))
}
