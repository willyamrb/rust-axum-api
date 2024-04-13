use axum::{
    extract::{Path, Query},
    http::{HeaderMap, HeaderValue},
    Json,
};
use axum_extra::{headers::UserAgent, TypedHeader};
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

pub async fn pathvar_test(Path(id): Path<i32>) -> String {
    id.to_string()
}

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    id: i32,
    message: String,
}

pub async fn query_test(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}

pub async fn headers_test(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}

pub async fn custom_header_test(headers: HeaderMap) -> String {
    let custom_headers: &HeaderValue = headers.get("x-custom-header").unwrap();
    let data = custom_headers.to_str().unwrap().to_owned();
    data
}
