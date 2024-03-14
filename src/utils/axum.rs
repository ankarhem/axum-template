pub mod extractors {
    use crate::prelude::AppError;
    use axum::extract::{
        rejection::{JsonRejection, PathRejection, QueryRejection},
        FromRequest, FromRequestParts,
    };

    #[derive(FromRequestParts)]
    #[from_request(via(axum::extract::Path), rejection(AppError))]
    pub struct AppPath<T>(pub T);
    impl From<PathRejection> for AppError {
        fn from(rejection: PathRejection) -> Self {
            tracing::debug!("PathRejection: {:?}", rejection.body_text());
            Self::new(axum::http::StatusCode::BAD_REQUEST, rejection.body_text())
        }
    }

    #[derive(FromRequestParts)]
    #[from_request(via(axum::extract::Query), rejection(AppError))]
    pub struct AppQuery<T>(pub T);
    impl From<QueryRejection> for AppError {
        fn from(rejection: QueryRejection) -> Self {
            tracing::debug!("QueryRejection: {:?}", rejection.body_text());
            Self::new(axum::http::StatusCode::BAD_REQUEST, rejection.body_text())
        }
    }

    #[derive(FromRequest)]
    #[from_request(via(axum::extract::Json), rejection(AppError))]
    pub struct AppJson<T>(pub T);
    impl From<JsonRejection> for AppError {
        fn from(rejection: JsonRejection) -> Self {
            tracing::debug!("JsonRejection: {:?}", rejection.body_text());
            Self::new(axum::http::StatusCode::BAD_REQUEST, rejection.body_text())
        }
    }
}
