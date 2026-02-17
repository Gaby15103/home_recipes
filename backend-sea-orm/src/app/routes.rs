use actix_files::Files;
use actix_web::web;

use crate::controllers::{
    auth_controller, files_controller, ingredients_controller, recipes_controller,
    upload_controller, users_controller,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(recipes_controller::configure)
            .configure(auth_controller::configure)
            .configure(users_controller::configure)
            .configure(ingredients_controller::configure)
            .configure(upload_controller::configure),
    )
    .configure(files_controller::configure);
}
