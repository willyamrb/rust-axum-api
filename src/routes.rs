use crate::controllers::{body_json_test, body_test, index, pathvar_test, query_test};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/body", post(body_test))
        .route("/json", post(body_json_test))
        .route("/pathvar/:id", post(pathvar_test))
        .route("/query", get(query_test))
}
