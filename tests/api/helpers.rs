use std::sync::LazyLock;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::{get_configurtion, DatabaseSettings};
use zero2prod::telemetry::{get_subscriber, init_subcsriber};
use zero2prod::startup::{Application, get_connection_pool};
use secrecy::Secret;

static TRACING: LazyLock<()> = LazyLock::new(|| {
  let default_filter_level = "info".to_string();
  let subscriber_name = "test".to_string();

  if std::env::var("TEST_LOG").is_ok() {
    let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
    init_subcsriber(subscriber);
  } else {
    let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
    init_subcsriber(subscriber);
  }
});

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool,
}

impl TestApp {
  pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
    reqwest::Client::new()
      .post(&format!("{}/subscriptions", &self.address))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(body)
      .send()
      .await
      .expect("Failed to execute request.")
  }
}

pub async fn spawn_app() -> TestApp {
  LazyLock::force(&TRACING);

  let configuration = {
    let mut c = get_configurtion().expect("Failed to read configuration.");
    c.database.database_name = Uuid::new_v4().to_string();
    c.application.port = 0;
    c
  };

  // Create and migrate the database
  configure_database(&configuration.database).await;

  // Launch the application as a background task
  let application = Application::build(configuration.clone())
    .await
    .expect("Failed to build application.");
  let address = format!("http://127.0.0.1:{}", application.port());
  let _ = tokio::spawn(application.run_until_stopped());

	TestApp { address, db_pool: get_connection_pool(&configuration.database) }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
	// Create database
	let maintenance_settings = DatabaseSettings {
		database_name: "postgres".to_string(),
		username: "postgres".to_string(),
		password: Secret::new("password".to_string()),
		host: config.host.clone(),
		port: config.port,
    require_ssl: config.require_ssl,
	};
	let mut connection = PgConnection::connect_with(
			&maintenance_settings.connect_options()
		)
		.await
		.expect("Failed to connect to Postgres");

  connection
		.execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
		.await
		.expect("Failed to create database.");

	// Migrate databse
	let connection_pool = PgPool::connect_with(config.connect_options())
		.await
		.expect("Failed to connect to Postgres.");
	sqlx::migrate!("./migrations")
		.run(&connection_pool)
		.await
		.expect("Failed to migrate database");

	connection_pool
}
