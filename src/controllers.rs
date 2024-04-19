use axum::{
    extract::{Path, Query},
    http::{HeaderMap, HeaderValue, StatusCode},
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

pub async fn error_route() -> Result<(), StatusCode> {
    Err(StatusCode::BAD_REQUEST)
}

#[derive(Serialize)]
pub struct JsData {
    message: String,
    username: String,
    count: i32,
}

pub async fn get_json() -> Json<JsData> {
    let data: JsData = JsData {
        message: "Hello, World!".to_string(),
        username: "John Doe".to_string(),
        count: 42,
    };

    Json(data)
}

#[derive(Deserialize)]
pub struct Credentials {
    username: Option<String>, //make it optional
    password: String,         //make it required
}

pub async fn login(Json(credentials): Json<Credentials>) -> String {
    format!(
        "Welcome, {}!",
        credentials.username.unwrap_or("User".to_string())
    )
}
