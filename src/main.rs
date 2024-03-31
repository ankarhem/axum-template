use PKG_NAME::{configuration::get_configuration, telemetry, AppState, Application};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber =
        telemetry::get_subscriber("PKG_NAME".into(), "debug,h2=warn".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let settings = get_configuration()?;
    let state = AppState::new()?;

    Application::build(settings, state)?
        .run_until_stopped()
        .await?;

    Ok(())
}
