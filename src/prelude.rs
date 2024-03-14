pub use crate::models::error::*;
pub use crate::utils::axum::extractors::*;

#[derive(Debug)]
pub struct Wrapper<T>(pub T);
