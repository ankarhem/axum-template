use crate::{prelude::*, AppState};

use axum::{extract::State, response::IntoResponse};
use error_stack::{report, Context, FutureExt, Report, ResultExt};
use serde::{Deserialize, Serialize};

#[tracing::instrument(skip(state), name = "random_number")]
#[axum::debug_handler]
pub async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let number = state
      .client
      .get("https://www.random.org/integers/?num=1&min=1&max=100&col=1&base=10&format=plain&rnd=new")
      .send()
      .await
      .change_context(GetRandomNumberError)
      .attach_printable("Failed to reach underlying service")?
      .text().await.change_context(GetRandomNumberError).attach_printable("Response body could not be read")
      .and_then(|text| text.trim().parse::<u16>().change_context(GetRandomNumberError).attach_printable(format!("could not parse \"{text}\"")))?;

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
