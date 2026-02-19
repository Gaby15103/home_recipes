use dotenvy::dotenv;

mod config;
mod app;
mod errors;
mod controllers;
mod services;
mod repositories;
mod dto;
mod utils;
mod openapi;
mod domain;

use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        unsafe { std::env::set_var("RUST_LOG", "app=debug,actix_web=info"); }
    }
    env_logger::init();

    let config = Config::from_env()
        .expect("Failed to load configuration");

    app::start(config).await
}
