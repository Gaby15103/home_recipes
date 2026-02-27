use actix_web::web;

use crate::controllers::{auth_controller, files_controller, ingredients_controller, languages_controller, ocr_controller, recipes_controller, tags_controller, unit_controller, upload_controller, users_controller};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(recipes_controller::configure)
            .configure(auth_controller::configure)
            .configure(users_controller::configure)
            .configure(ingredients_controller::configure)
            .configure(tags_controller::configure)
            .configure(languages_controller::configure)
            .configure(unit_controller::configure)
            .configure(ocr_controller::configure)
            .configure(upload_controller::configure)
            .configure(files_controller::configure)
    );    
}
