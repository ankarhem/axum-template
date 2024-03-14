use std::net::TcpListener;
use PKG_NAME::telemetry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = telemetry::get_subscriber("PKG_NAME".into(), "debug".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    PKG_NAME::run(listener).await?;

    Ok(())
}
