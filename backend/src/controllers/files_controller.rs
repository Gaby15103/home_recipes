use actix_files::Files;
use actix_web::web;
use crate::controllers::users_controller::{change_password, get_me, get_sessions, update_profile};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Files::new("/assets", "./assets").show_files_listing()
    );
}