use std::fmt::Debug;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub struct Wrapper<T>(pub T);

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    message: String,
}

#[derive(Debug)]
pub struct AppError {
    code: StatusCode,
    message: Option<String>,
}

impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application Error")
    }
}

impl AppError {
    pub fn new<T: Into<String>>(code: StatusCode, message: T) -> Self {
        Self {
            code,
            message: Some(message.into()),
        }
    }

    pub fn bad_request<T: Into<String>>(message: T) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: Some(message.into()),
        }
    }

    pub fn not_found() -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message: None,
        }
    }

    pub fn server_error() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: None,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self.message {
            Some(message) => (self.code, Json(ErrorResponse { message })).into_response(),
            None => self.code.into_response(),
        }
    }
}

// When implicitly converting with `?` log as an error
// and convert to a 500 Internal Server Error
impl<C> From<error_stack::Report<C>> for AppError {
    fn from(err: error_stack::Report<C>) -> Self {
        tracing::error!("{:?}", err);
        Self::server_error()
    }
}
