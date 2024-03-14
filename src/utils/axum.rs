pub mod extractors {
    use crate::prelude::*;
    use axum::{
        extract::{
            rejection::{JsonRejection, PathRejection, QueryRejection},
            FromRequest, FromRequestParts,
        },
        response::IntoResponse,
    };
    use serde::Serialize;

    #[derive(FromRequestParts)]
    #[from_request(via(axum::extract::Path), rejection(AppError))]
    pub struct Path<T>(pub T);
    impl From<PathRejection> for AppError {
        fn from(rejection: PathRejection) -> Self {
            tracing::debug!("PathRejection: {:?}", rejection.body_text());
            Self::new(rejection.status(), rejection.body_text())
        }
    }

    #[derive(FromRequestParts)]
    #[from_request(via(axum::extract::Query), rejection(AppError))]
    pub struct Query<T>(pub T);
    impl From<QueryRejection> for AppError {
        fn from(rejection: QueryRejection) -> Self {
            tracing::debug!("QueryRejection: {:?}", rejection.body_text());
            Self::new(rejection.status(), rejection.body_text())
        }
    }

    #[derive(FromRequest)]
    #[from_request(via(axum::extract::Json), rejection(AppError))]
    pub struct Json<T>(pub T);
    impl From<JsonRejection> for AppError {
        fn from(rejection: JsonRejection) -> Self {
            tracing::debug!("JsonRejection: {:?}", rejection.body_text());
            Self::new(rejection.status(), rejection.body_text())
        }
    }
    impl<T: Serialize> IntoResponse for Json<T> {
        fn into_response(self) -> axum::http::Response<axum::body::Body> {
            axum::Json(self.0).into_response()
        }
    }
}
