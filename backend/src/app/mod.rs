use crate::db::{new_pool, DbExecutor};
use actix::prelude::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_web::{error, http::header::{CONTENT_TYPE}, middleware::Logger, web, web::Data, App, HttpRequest, HttpResponse, HttpServer};
use std::env;
use actix_files::Files;
use actix_multipart::form::MultipartFormConfig;

mod profiles;
pub mod users;
pub mod tags;
pub mod recipes;
pub mod debug;
pub mod two_factor;

/// Represents the shared application state, including the database executor.
pub struct AppState {
    pub db: Addr<DbExecutor>,
}

/// A simple index route returning a "Hello world!" message.
fn index(_state: Data<AppState>, _req: HttpRequest) -> &'static str {
    "Hello world!"
}

/// Custom error handler for JSON payload errors.
/// 
/// Matches broad categories of JSON errors and returns appropriate HTTP responses.
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

/// Custom error handler for query parameter errors.
/// 
/// Matches broad categories of query errors and returns appropriate HTTP responses.
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

/// Initializes and starts the Actix web server.
/// 
/// Configures the server with routes, middleware, and application state.
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
/* ----------------------------- ROUTE TREE ----------------------------- */

fn routes(app: &mut web::ServiceConfig) {
    app
        .service(api_routes())
        .service(debug_routes())
        .service(static_routes());

}

/* ----------------------------- STATIC ----------------------------- */

fn static_routes() -> actix_web::Scope {
    web::scope("")
        .service(Files::new("/assets", "./assets").show_files_listing())
}

/* ----------------------------- API ROOT ----------------------------- */

fn api_routes() -> actix_web::Scope {
    web::scope("/api")
        .service(auth_routes())
        .service(user_routes())
        .service(recipe_routes())
        .service(tag_routes())
        .service(two_factor_routes())
}

/* ----------------------------- AUTH ----------------------------- */

fn auth_routes() -> actix_web::Scope {
    web::scope("/auth")
        .route("/register", web::post().to(users::register))
        .route("/login", web::post().to(users::login))
        .route("/logout", web::post().to(users::logout))
}

/* ----------------------------- USERS ----------------------------- */

fn user_routes() -> actix_web::Scope {
    web::scope("/users")
        .route("/me", web::get().to(users::get_current))
        .route("/me", web::put().to(users::update))
}

/* ----------------------------- TAGS ----------------------------- */

fn tag_routes() -> actix_web::Scope {
    web::scope("/tags")
        .route("", web::get().to(tags::list))
        .route("", web::post().to(tags::create))
        .route("/{id}", web::put().to(tags::update))
        //.route("/{id}", web::delete().to(tags::delete))
}

/* ----------------------------- RECIPES ----------------------------- */

fn recipe_routes() -> actix_web::Scope {
    web::scope("/recipes")
        // collection
        .route("", web::get().to(recipes::list))
        .route("/by_page", web::get().to(recipes::get_by_page))
        .route("", web::post().to(recipes::create))

        // single recipe
        .route("/{id}", web::get().to(recipes::get))
        .route("/{id}", web::put().to(recipes::update))
        .route("/{id}", web::delete().to(recipes::delete))

        // analytics (aggregated)
        .route("/{id}/analytics", web::get().to(recipes::analytics))
        .route("/{id}/views", web::post().to(recipes::track_view))

        // favorites
        .route("/{id}/favorite", web::post().to(recipes::favorite))
        .route("/favorites", web::get().to(recipes::get_favorites))

        // ratings
        .route("/{id}/rating", web::post().to(recipes::rate))
        .route("/{id}/rating", web::delete().to(recipes::unrate))
        .route("/{id}/rating", web::get().to(recipes::get_rating))

        // comments (tree-based)
        .route("/{id}/comments", web::get().to(recipes::get_comments))
        .route("/{id}/comments", web::post().to(recipes::add_comment))

        // recipe version
        .route("/{recipe_id}/versions/{version_id}/restore", web::post().to(recipes::restore_version))

}
/* ----------------------------- Two-Factor ----------------------------- */
pub fn two_factor_routes() -> actix_web::Scope {
    web::scope("/two-factor")
        .route("/qr-code", web::get().to(two_factor::qr_code))
        .route("/secret-key", web::get().to(two_factor::secret_key))
        .route("/recovery-codes", web::get().to(two_factor::recovery_codes))
        .route("/enable", web::post().to(two_factor::enable))
        .route("/disable", web::post().to(two_factor::disable))
        .route("/status", web::get().to(two_factor::status))
        .route("/verify", web::post().to(two_factor::verify))
}

/* ----------------------------- DEBUG ----------------------------- */

fn debug_routes() -> actix_web::Scope {
    web::scope("/api/debug")
        .route("", web::post().to(debug::debug_multipart_parsed))
}