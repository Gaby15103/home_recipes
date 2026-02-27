use actix_files::Files;
use actix_web::web;
use std::env;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let file_url = env::var("FILE_URL").unwrap_or_else(|_| "./assets".to_string());
    println!("Static files being served from: {}", file_url);
    cfg.service(
        Files::new("/assets", file_url).show_files_listing()
    );
}