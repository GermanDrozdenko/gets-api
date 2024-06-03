mod autotests;
mod config;
mod entities;
mod handlers;

use std::io;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use config::{AppConfig, AppState};
use env_logger::Env;
use sea_orm::Database;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = AppConfig::from_env();
    let conn = Database::connect(config.database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { conn: conn.clone() }))
            .configure(config::config_handlers)
            .wrap(Logger::default())
            .wrap(AppConfig::set_cors(&config.client_url))
    })
    .bind(config.server_url)?
    .run()
    .await
}
