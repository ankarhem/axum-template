use std::net::{SocketAddr, TcpListener};
use std::time::Duration;

mod handlers;
mod prelude;
pub mod telemetry;
mod utils;

use axum::http::Method;
use axum::{body::Body, http::Request, routing, Router};
use error_stack::{Context, Result, ResultExt};
use once_cell::sync::Lazy;
use reqwest::Client;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::info_span;

#[derive(Clone)]
pub struct AppState {
    client: reqwest::Client,
}

fn app() -> Result<Router, InitializeAppError> {
    let reqwest_client = create_client()?;

    let app_state = AppState {
        client: reqwest_client,
    };

    let router = Router::new()
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                // allow `GET` and `POST` when accessing the resource
                .allow_methods([Method::GET, Method::POST])
                // allow requests from any origin
                .allow_origin(Any),
        )
        .layer(
            // Let's create a tracing span for each request
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
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
                    uri = %request.uri(),
                )
            }),
        )
        .layer(RequestIdLayer)
        .with_state(app_state)
        // Omit these from the logs etc.
        .route(
            "/__healthcheck",
            routing::get(handlers::healthcheck::handler),
        );

    Ok(router)
}

#[derive(Debug)]
pub struct InitializeAppError;

impl std::fmt::Display for InitializeAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to initialize app")
    }
}

impl Context for InitializeAppError {}

pub async fn run(std_listener: TcpListener) -> Result<(), InitializeAppError> {
    let addr = std_listener
        .local_addr()
        .change_context(InitializeAppError)?;

    std_listener
        .set_nonblocking(true)
        .change_context(InitializeAppError)?;
    let listener =
        tokio::net::TcpListener::from_std(std_listener).change_context(InitializeAppError)?;

    tracing::info!("Listening on {}", addr);

    let app = app()?;
    axum::serve(listener, app)
        // axum::Server::from_tcp(listener)?
        //     .serve(app().into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install CTRL+C signal handler");
        })
        .await
        .change_context(InitializeAppError)?;

    Ok(())
}

// test helpers
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber =
            telemetry::get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        telemetry::init_subscriber(subscriber);
    } else {
        let subscriber =
            telemetry::get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        telemetry::init_subscriber(subscriber);
    }
});

pub fn spawn_app() -> SocketAddr {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("To bind to random port");
    let addr = listener.local_addr().expect("To get local address");

    tokio::spawn(run(listener));

    addr
}

fn create_client() -> Result<Client, InitializeAppError> {
    Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(reqwest::header::USER_AGENT, "PKG_NAME".parse().unwrap());

            headers
        })
        .pool_idle_timeout(Duration::from_secs(15))
        .pool_max_idle_per_host(10)
        .build()
        .change_context(InitializeAppError)
}
