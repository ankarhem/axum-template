use PKG_NAME::{configuration::get_configuration, telemetry, AppState, Application};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_configuration()?;

    let subscriber = telemetry::get_subscriber(
        "PKG_NAME".into(),
        "debug,h2=warn".into(),
        &config.telemetry,
        std::io::stdout,
    );
    telemetry::init_subscriber(subscriber);

    let state = AppState::new()?;

    Application::build(config, state)?
        .run_until_stopped()
        .await?;

    Ok(())
}
