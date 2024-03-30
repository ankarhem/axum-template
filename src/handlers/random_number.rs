use std::sync::Arc;

use crate::{clients::RandomNumberService, prelude::*};

use axum::{extract::State, response::IntoResponse};
use error_stack::Context;

#[tracing::instrument(skip(service), name = "random_number")]
// #[axum::debug_handler]
pub async fn get(
    State(service): State<Arc<dyn RandomNumberService>>,
) -> Result<impl IntoResponse, AppError> {
    let number = service.get_random_number().await?;

    tracing::info!("Got random number: {}", number);

    number
        .checked_mul(100)
        .map(|n| n.to_string())
        .ok_or(AppError::server_error())
}

#[derive(Debug)]
struct GetRandomNumberError;

impl std::fmt::Display for GetRandomNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("GetRandomNumberError")
    }
}

impl Context for GetRandomNumberError {}
