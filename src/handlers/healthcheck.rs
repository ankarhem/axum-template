use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}
