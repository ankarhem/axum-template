use anyhow::Result;
use std::net::TcpListener;
use PKG_NAME::telemetry;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = telemetry::get_subscriber("unpkg".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;
    PKG_NAME::run(listener).await?;

    Ok(())
}
