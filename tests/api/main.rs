mod health_check;
mod random_number;

mod helpers {
    use once_cell::sync::Lazy;
    use std::net::SocketAddr;
    use PKG_NAME::{
        configuration::{get_configuration, Settings},
        telemetry::{get_subscriber, init_subscriber},
        AppState,
    };

    static TRACING: Lazy<()> = Lazy::new(|| {
        let default_filter_level = "info".to_string();
        let subscriber_name = "test".to_string();
        if std::env::var("TEST_LOG").is_ok() {
            let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
            init_subscriber(subscriber);
        } else {
            let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
            init_subscriber(subscriber);
        };
    });

    pub struct TestApp {
        addr: SocketAddr,
        client: reqwest::Client,
    }

    impl TestApp {
        pub async fn spawn(state: AppState) -> Self {
            Lazy::force(&TRACING);

            let mut settings = get_configuration().expect("Failed to read configuration.");
            settings.application.port = 0;

            let application = PKG_NAME::Application::build(settings, state)
                .expect("Failed to build application.");

            let addr = application.addr().expect("Failed to get address.");

            let _ = tokio::spawn(application.run_until_stopped());

            let client = reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .expect("Failed to create reqwest client.");

            Self {
                addr,
                client: client,
            }
        }
    }

    impl TestApp {
        pub async fn get_healthcheck(&self) -> reqwest::Response {
            self.client
                .get(format!("http://{}/__healthcheck", self.addr))
                .send()
                .await
                .expect("Failed to execute request.")
        }

        pub async fn get_random_number(&self) -> reqwest::Response {
            self.client
                .get(format!("http://{}/random_number", self.addr))
                .send()
                .await
                .expect("Failed to execute request.")
        }
    }
}
