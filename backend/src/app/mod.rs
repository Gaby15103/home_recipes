use crate::db::{new_pool, DbExecutor};
use actix::prelude::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE}, middleware::Logger, web,
    web::Data,
    App,
    HttpRequest,
    HttpServer,
};
use std::env;
mod profiles;
pub mod users;
pub mod tags;
pub mod recipes;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

fn index(_state: Data<AppState>, _req: HttpRequest) -> &'static str {
    "Hello world!"
}

pub async fn start() -> std::io::Result<()> {
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = new_pool(database_url).expect("Failed to create pool.");
    let database_address =
        SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    println!("You can access the server at {}", bind_address); // <- print BEFORE running

    HttpServer::new(move || {
        let state = AppState {
            db: database_address.clone(),
        };
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173") // your frontend URL
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);
        App::new()
            .app_data(Data::new(state))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(routes)
    })
    .bind(&bind_address)?
    .run()
    .await?;

    Ok(())
}
fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/")).service(
        web::scope("/api")
            .service(web::resource("user/register")
                .route(web::post().to(users::register)))
            .service(web::resource("user/login")
            .   route(web::post().to(users::login)))
            .service(web::resource("user")
                .route(web::get().to(users::get_current))
                .route(web::put().to(users::update)))
            .service(web::resource("tag/create")
                .route(web::post().to(tags::create)))
            .service(web::resource("tag/update")
                .route(web::put().to(tags::update)))
            .service(web::resource("recipe/create")
                .route(web::post().to(recipes::create)))
            .service(web::resource("recipe/update")
                .route(web::put().to(recipes::update)))
            .service(web::resource("recipe/get_all")
                .route(web::get().to(recipes::get_all)))
    );
}
