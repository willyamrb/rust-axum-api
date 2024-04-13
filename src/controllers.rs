use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn index() -> &'static str {
    "Hello, World!"
}

pub async fn body_test(body: String) -> String {
    body
}

#[derive(Deserialize, Serialize)]
pub struct Mirror {
    message: String,
}

pub async fn body_json_test(Json(body): Json<Mirror>) -> Json<Mirror> {
    Json(body)
}
