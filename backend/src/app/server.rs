use actix_web::{App, HttpServer, web::Data};
use sea_orm::Database;
use std::sync::Arc;
use actix_web::middleware::from_fn;
use redis::Client;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use migration::{Migrator, MigratorTrait};
use crate::app::middleware::auth_middleware;
use crate::config::Config;
use crate::errors;
use crate::openapi::ApiDoc;
use super::state::AppState;
use super::{routes, middleware};

pub async fn start(config: Config) -> std::io::Result<()> {

    let config = Arc::new(config);

    let db = Database::connect(&config.database_url)
        .await
        .expect("DB connection failed");
    Migrator::up(&db, None).await.expect("TODO: panic message");

    let redis = Arc::new(
        Client::open(config.redis_url.clone())
            .expect("Redis connection failed")
    );

    let bind_address = config.bind_address.clone();


    println!("You can access the server at {}", config.bind_address);

    HttpServer::new(move || {

        let state = Data::new(AppState {
            db: db.clone(),
            redis: redis.clone(),
            config: config.clone(),
        });

        App::new()
            .app_data(state)
            .wrap(middleware::cors())
            .wrap(actix_web::middleware::Logger::default())
            .app_data(
                actix_web::web::JsonConfig::default()
                    .error_handler(errors::json_error_handler)
            )
            .app_data(
                actix_web::web::QueryConfig::default()
                    .error_handler(errors::query_error_handler)
            )
            .configure(routes::configure)
            .wrap(from_fn(auth_middleware))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi())
                    .config(utoipa_swagger_ui::Config::default().try_it_out_enabled(true))
            )
    })
        .bind(bind_address)?
        .run()
        .await
}

