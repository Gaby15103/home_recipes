#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate validator_derive;

mod app;
mod db;
mod error;
mod models;
mod prelude;
mod schema;
mod utils;
mod dto;

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        unsafe { env::set_var("RUST_LOG", "conduit=debug,actix_web=info"); }
    }
    env_logger::init();

    app::start().await
}
