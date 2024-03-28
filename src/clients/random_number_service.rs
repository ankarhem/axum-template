use axum::async_trait;
use error_stack::{Context, Report, ResultExt};

#[async_trait]
pub trait RandomNumberService: Send + Sync {
    async fn get_random_number(&self) -> Result<u16, Report<RandomNumberServiceError>>;
}

pub struct RandomNumberServiceClient {
    pub(crate) inner: reqwest::Client,
}

#[async_trait]
impl RandomNumberService for RandomNumberServiceClient {
    async fn get_random_number(&self) -> Result<u16, Report<RandomNumberServiceError>> {
        self.inner.get("https://www.random.org/integers/?num=1&min=1&max=100&col=1&base=10&format=plain&rnd=new")
        .send()
        .await
        .change_context(RandomNumberServiceError)
        .attach_printable("Failed to reach underlying service")?
        .text().await.change_context(RandomNumberServiceError).attach_printable("Response body could not be read")
        .and_then(|text| text.trim().parse::<u16>().change_context(RandomNumberServiceError).attach_printable(format!("could not parse \"{text}\"")))
    }
}

#[derive(Debug)]
pub struct RandomNumberServiceError;

impl std::fmt::Display for RandomNumberServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RandomNumberServiceError")
    }
}

impl Context for RandomNumberServiceError {}
