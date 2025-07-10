use zero2prod::configuration::get_configurtion;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subcsriber};
use sqlx::postgres::PgPool;
use std::{net::TcpListener, ops::Sub};
use tracing::{subscriber::{self, set_global_default}, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

// pub fn get_subscriber(
//     name: String,
//     env_filter: String
// ) -> impl Subscriber + Send + Sync {
//     let env_filter = EnvFilter::try_from_default_env()
//         .unwrap_or(EnvFilter::new(env_filter));
//     let formatting_layer = BunyanFormattingLayer::new(
//         name,
//         std::io::stdout
//     );
//     Registry::default()
//         .with(env_filter)
//         .with(JsonStorageLayer)
//         .with(formatting_layer)
// }

// pub fn init_subcsriber(subscriber: impl Subscriber + Send + Sync) {
//     LogTracer::init().expect("Failed to set logger");
//     set_global_default(subscriber).expect("Failed to set subscriber");
// }

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subcsriber(subscriber);

    let configuration = get_configurtion().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(
            &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}