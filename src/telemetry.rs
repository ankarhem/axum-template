use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace, Resource};
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

use crate::configuration::TelemetrySettings;

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    config: &TelemetrySettings,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let fmt_layer = tracing_subscriber::fmt::layer();

    let endpoint = &config.endpoint;
    let resource_map = Resource::new(vec![KeyValue::new("service.name", name.clone())]);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(trace::config().with_resource(resource_map))
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint)
                .with_protocol(opentelemetry_otlp::Protocol::HttpBinary),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("Couldn't create OTLP tracer");

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let error_layer = tracing_error::ErrorLayer::default();

    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .with(error_layer)
        .with(telemetry_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect log events to tracer
    // tracing_log::LogTracer::init().expect("To init log tracer");
    set_global_default(subscriber).expect("To set global default subscriber");
}
