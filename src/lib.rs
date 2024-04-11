use axum::Router;
use configuration::Settings;
use error_stack::{Context, Report, ResultExt};
use std::net::SocketAddr;

pub mod clients;
pub mod configuration;
mod models;
mod prelude;
mod router;
pub mod telemetry;
mod utils;

pub use models::state::AppState;
pub use router::app;

pub struct Application {
    listener: std::net::TcpListener,
    app: Router,
}

impl Application {
    pub fn build(config: Settings, state: AppState) -> Result<Self, Report<InitializeAppError>> {
        let port = config.application.port;
        let host = config.application.host;
        let listener = std::net::TcpListener::bind(format!("{host}:{port}"))
            .change_context(InitializeAppError)
            .attach_printable("could not bind to address")?;

        let app = app(state);

        Ok(Self { listener, app })
    }
}

impl Application {
    pub fn addr(&self) -> std::io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    pub async fn run_until_stopped(self) -> Result<(), Report<InitializeAppError>> {
        let addr = self.addr().change_context(InitializeAppError)?;
        self.listener
            .set_nonblocking(true)
            .change_context(InitializeAppError)?;
        let tokio_listener =
            tokio::net::TcpListener::from_std(self.listener).change_context(InitializeAppError)?;

        tracing::info!("Listening on {}", addr);

        axum::serve(tokio_listener, self.app.into_make_service())
            .with_graceful_shutdown(async {
                tokio::signal::ctrl_c()
                    .await
                    .expect("Failed to install CTRL+C signal handler");
            })
            .await
            .change_context(InitializeAppError)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct InitializeAppError;

impl std::fmt::Display for InitializeAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InitializeAppError")
    }
}

impl Context for InitializeAppError {}
