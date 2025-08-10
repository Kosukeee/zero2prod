// use sqlx::postgres::PgPoolOptions;
// use std::net::TcpListener;
use zero2prod::configuration::get_configurtion;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subcsriber};
// use zero2prod::email_client::EmailClient;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "zero2prod".into(), "info".into(), std::io::stdout
    );
    init_subcsriber(subscriber);

    let configuration = get_configurtion().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}