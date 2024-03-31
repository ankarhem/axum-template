use axum::{body::Body, extract::MatchedPath, http::Request, routing, Router};
use reqwest::Method;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::info_span;

use crate::AppState;

mod error_test;
mod healthcheck;
mod random_number;

pub fn app(state: AppState) -> Router {
    let router = Router::new()
        .route("/random_number", routing::get(random_number::get))
        .route("/error_test", routing::get(error_test::get))
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                // allow `GET` and `POST` when accessing the resource
                .allow_methods([Method::GET, Method::POST])
                // allow requests from any origin
                .allow_origin(Any),
        )
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);
                // We get the request id from the extensions
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                // And then we put it along with other information into the `request` span
                info_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    path = ?matched_path,
                )
            }),
        )
        .layer(RequestIdLayer)
        .with_state(state)
        // Omit these from the logs etc.
        .route("/__healthcheck", routing::get(healthcheck::get));

    router
}
