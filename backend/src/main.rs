use dotenvy::dotenv;

mod app;
mod config;
mod controllers;
mod domain;
mod dto;
mod errors;
mod openapi;
mod repositories;
mod services;
mod utils;
mod recipe_parser;
mod logging;

use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "app=debug,actix_web=info");
        }
    }
    if std::env::var("ERROR_LOG_DIR").is_err() {
        unsafe {
            std::env::set_var("ERROR_LOG_DIR", "./logs/errors");
        }
    }
    env_logger::init();

    let config = Config::from_env().expect("Failed to load configuration");

    app::start(config).await
}
