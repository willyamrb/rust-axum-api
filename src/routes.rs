use crate::controllers::{body_json_test, body_test, index};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/body", post(body_test))
        .route("/json", post(body_json_test))
}
