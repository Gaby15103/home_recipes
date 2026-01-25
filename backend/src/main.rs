#[macro_use]
extern crate diesel;
#[macro_use]
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate validator;

mod app;
mod db;
mod error;
mod models;
mod prelude;
mod schema;
mod utils;
mod dto;
mod config;

use std::env;
use dotenvy::dotenv;
use config::Config;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
        }
    }
    env_logger::init();

    let config = Config::from_env();

    app::start(config.unwrap()).await
}
