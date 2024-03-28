pub use crate::models::error::*;
pub use crate::utils::axum::extractors::*;

#[derive(Debug, Clone)]
pub struct Wrapper<T>(pub T);

impl<T> std::ops::Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
