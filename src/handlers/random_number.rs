use crate::{models::state::AppState, prelude::*};

use axum::{extract::State, response::IntoResponse};
use error_stack::Context;

#[tracing::instrument(skip(state), name = "random_number")]
// #[axum::debug_handler]
pub async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let number = state.random_number_service.get_random_number().await?;

    tracing::info!("External number fetched: {}", number);

    Ok((number * 100).to_string())
}

#[derive(Debug)]
struct GetRandomNumberError;

impl std::fmt::Display for GetRandomNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("GetRandomNumberError")
    }
}

impl Context for GetRandomNumberError {}
