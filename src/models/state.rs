use std::{sync::Arc, time::Duration};

use axum::extract::FromRef;
use error_stack::{Context, Report, ResultExt};
use reqwest::Client;

use crate::clients::{RandomNumberService, RandomNumberServiceClient};

#[derive(Clone)]
pub struct AppState {
    pub random_number_service: Arc<dyn RandomNumberService>,
}

impl AppState {
    pub fn new() -> Result<Self, Report<CreateAppStateError>> {
        let client = create_client()?;
        Ok(Self {
            random_number_service: Arc::new(RandomNumberServiceClient::new(client)),
        })
    }
}

impl AppState {
    pub fn replace_random_number_service(self, service: Arc<dyn RandomNumberService>) -> Self {
        Self {
            random_number_service: service,
            ..self
        }
    }
}

impl FromRef<AppState> for Arc<dyn RandomNumberService> {
    fn from_ref(state: &AppState) -> Self {
        state.random_number_service.clone()
    }
}

fn create_client() -> Result<Client, Report<CreateAppStateError>> {
    Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(reqwest::header::USER_AGENT, "PKG_NAME".parse().unwrap());

            headers
        })
        .pool_idle_timeout(Duration::from_secs(15))
        .pool_max_idle_per_host(10)
        .build()
        .change_context(CreateAppStateError)
}

#[derive(Debug)]
pub struct CreateAppStateError;

impl std::fmt::Display for CreateAppStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CreateAppStateError")
    }
}

impl Context for CreateAppStateError {}
