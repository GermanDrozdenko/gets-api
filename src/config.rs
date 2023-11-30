use std::env;

use crate::handlers;

use actix_web::web;
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub conn: DatabaseConnection,
}

pub struct AppConfig {
    pub server_url: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> AppConfig {
        dotenv().ok();

        AppConfig {
            server_url: env::var("SERVER_URL").expect("SERVER_URL must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}

pub fn config_handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::check).service(handlers::get_all_beer);
}
