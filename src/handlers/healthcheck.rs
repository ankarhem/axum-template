use crate::prelude::*;

use axum::response::IntoResponse;
use serde_json::json;

pub async fn get() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}
