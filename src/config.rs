use std::env;

use crate::handlers;

use actix_cors::Cors;
use actix_web::{http, web};
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub conn: DatabaseConnection,
}

#[derive(Clone)]
pub struct AppConfig {
    pub server_url: String,
    pub client_url: String,
    pub database_url: String,
    pub home_dir: String,
}

impl AppConfig {
    pub fn from_env() -> AppConfig {
        dotenv().ok();

        AppConfig {
            server_url: env::var("SERVER_URL").expect("SERVER_URL must be set"),
            client_url: env::var("CLIENT_URL").expect("CLIENT_URL must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            home_dir: env::var("HOME_DIR").expect("HOME_DIR must be set"),
        }
    }

    pub fn set_cors(client_url: &String) -> Cors {
        Cors::default()
            .allowed_origin(format!("http://{}", client_url).as_str())
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600)
    }
}

pub fn config_handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::check)
        .service(handlers::get_all_beer)
        .service(handlers::get_result_beer)
        .service(handlers::get_random_beer)
        .service(handlers::get_file);
}
