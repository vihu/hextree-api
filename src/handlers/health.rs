use axum::{http::StatusCode, Json};
use serde_json::Value;

pub async fn health() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json("healthy".into()))
}
