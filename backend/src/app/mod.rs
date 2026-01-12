use crate::db::{new_pool, DbExecutor};
use actix::prelude::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_web::{error, http::header::{AUTHORIZATION, CONTENT_TYPE}, middleware::Logger, web, web::Data, App, HttpRequest, HttpResponse, HttpServer};
use std::env;
use actix_files::Files;
use actix_multipart::form::MultipartFormConfig;

mod profiles;
pub mod users;
pub mod tags;
pub mod recipes;
mod debug;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

fn index(_state: Data<AppState>, _req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn json_error_handler(err: error::JsonPayloadError, _req: &actix_web::HttpRequest) -> error::Error {
    let detail = err.to_string();

    // Match broad categories
    let response = match &err {
        error::JsonPayloadError::ContentType => {
            HttpResponse::BadRequest().body(format!("Invalid content type: {}", detail))
        }
        error::JsonPayloadError::Deserialize(inner) => {
            // inner contains serde_json errors
            HttpResponse::BadRequest().body(format!("JSON deserialization error: {}", inner))
        }
        error::JsonPayloadError::Payload(p) => {
            HttpResponse::BadRequest().body(format!("Payload error: {}", p))
        }
        _ => HttpResponse::BadRequest().body(format!("Bad request: {}", detail)),
    };

    error::InternalError::from_response(err, response).into()
}

fn query_error_handler(
    err: error::QueryPayloadError,
    _req: &actix_web::HttpRequest,
) -> error::Error {
    let detail = err.to_string();

    let response = match &err {
        error::QueryPayloadError::Deserialize(inner) => {
            HttpResponse::BadRequest()
                .body(format!("Invalid query parameter: {}", inner))
        }
        _ => HttpResponse::BadRequest()
            .body(format!("Bad query string: {}", detail)),
    };

    error::InternalError::from_response(err, response).into()
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
            .allowed_headers(vec![CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);
        App::new()
            .app_data(Data::new(state))
            .app_data(MultipartFormConfig::default()
                .total_limit(100 * 1024 * 1024)
                .memory_limit(10 * 1024 * 1024)
            )
            .app_data(web::JsonConfig::default()
                .error_handler(json_error_handler))
            .app_data(
                web::QueryConfig::default()
                    .error_handler(query_error_handler)
            )
            .app_data(web::PayloadConfig::new(100 * 1024 * 1024))
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
    app
        .service(debug_routes())
        .service(api_routes())
        .service(static_routes());
}

fn static_routes() -> actix_web::Scope {
    web::scope("")
        .service(Files::new("/assets", "./assets")
            .show_files_listing())
}


fn api_routes() -> actix_web::Scope {
    web::scope("/api")
        .service(user_routes())
        .service(recipe_routes())
        .service(tag_routes())
}


fn debug_routes() -> actix_web::Scope {
    web::scope("/api/debug")
        .route("", web::post().to(debug::debug_multipart_parsed))
}

fn user_routes() -> actix_web::Scope {
    web::scope("/user")
        .route("/register", web::post().to(users::register))
        .route("/login", web::post().to(users::login))
        .route("/logout", web::post().to(users::logout))
        .route("", web::get().to(users::get_current))
        .route("edit", web::put().to(users::update))
}

fn tag_routes() -> actix_web::Scope {
    web::scope("/tag")
        .route("/create", web::post().to(tags::create))
        .route("/update", web::put().to(tags::update))
        .route("/get_all", web::get().to(tags::get_all))
}

fn recipe_routes() -> actix_web::Scope {
    web::scope("/recipe")
        // STATIC ROUTES FIRST
        .route("/get_all", web::get().to(recipes::get_all))
        .route("/get_by_page", web::get().to(recipes::get_by_page))
        .route("/create", web::post().to(recipes::create))
        .route("/update", web::put().to(recipes::update))

        // DYNAMIC ROUTES LAST
        .route("/{id}", web::get().to(recipes::get_by_id))
}
