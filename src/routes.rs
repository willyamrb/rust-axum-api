use crate::controllers::{
    body_json_test, body_test, custom_header_test, error_route, headers_test, index, pathvar_test,
    query_test,
};
use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET])
        .allow_origin(Any);

    Router::new()
        .route("/", get(index))
        .route("/body", post(body_test))
        .route("/json", post(body_json_test))
        .route("/pathvar/:id", post(pathvar_test))
        .route("/query", get(query_test))
        .route("/headers", get(headers_test))
        .route("/custom-headers", get(custom_header_test))
        .route("/error", get(error_route))
        .layer(cors)
}
