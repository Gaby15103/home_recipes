use actix_files::Files;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Files::new("/assets", "./assets").show_files_listing()
    );
}