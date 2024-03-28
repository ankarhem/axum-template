use std::sync::Arc;

use crate::{
    clients::{RandomNumberService, RandomNumberServiceClient},
    create_client,
};

#[derive(Clone)]
pub struct AppState {
    client: reqwest::Client,
    pub(crate) random_number_service: Arc<dyn RandomNumberService>,
}

impl Default for AppState {
    fn default() -> Self {
        let client = create_client().unwrap();
        Self {
            client: client.clone(),
            random_number_service: Arc::new(RandomNumberServiceClient {
                inner: client.clone(),
            }),
        }
    }
}
