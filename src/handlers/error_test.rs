use crate::prelude::*;

use axum::response::IntoResponse;
use error_stack::{report, Context, Report};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    first: String,
    last: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Other {
    favorite_number: i32,
}

#[tracing::instrument(name = "error_test")]
#[axum::debug_handler]
pub async fn get(
    Query(name): Query<Name>,
    Query(other): Query<Other>,
) -> Result<impl IntoResponse, AppError> {
    let favorite_number = other.favorite_number;

    if favorite_number < 0 {
        return Err(AppError::bad_request("Favorite number must be positive"));
    }

    if name.first == "" || name.last == "" {
        return Err(AppError::bad_request(
            "First and last name must not be empty",
        ));
    }

    is_secret_number(&other.favorite_number)?;

    Ok(format!("Hello, {} {}!", name.first, name.last))
}

#[derive(Debug)]
struct CheckNumberError;

impl std::fmt::Display for CheckNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CheckNumberError")
    }
}

impl Context for CheckNumberError {}

#[tracing::instrument(name = "some_func")]
pub fn is_secret_number(number: &i32) -> Result<(), Report<CheckNumberError>> {
    if *number != 1337 {
        return Err(report!(CheckNumberError)
            .attach_printable(format!("`{}` is not the secret number", number)));
    }
    Ok(())
}
