use std::sync::Arc;

use crate::{
    clients::{RandomNumberService, RandomNumberServiceClient},
    create_client,
};

#[derive(Clone)]
pub struct AppState {
    pub random_number_service: Arc<dyn RandomNumberService>,
}

impl Default for AppState {
    fn default() -> Self {
        let client = create_client().unwrap();
        Self {
            random_number_service: Arc::new(RandomNumberServiceClient::new(client)),
        }
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
