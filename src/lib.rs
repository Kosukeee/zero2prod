use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

pub mod configuration;
pub mod domain;
pub mod email_client;
pub mod routes;
pub mod startup;
pub mod telemetry;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct FormData {
  email: String,
  name: String,
}

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
      App::new()
          .route("/health_check", web::get().to(health_check))
          .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
  Ok(server)
}