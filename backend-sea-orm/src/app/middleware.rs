use actix_cors::Cors;
use actix_web::http::header::CONTENT_TYPE;

pub fn cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:5173")
        .allowed_methods(vec!["GET","POST","PUT","DELETE","OPTIONS"])
        .allowed_headers(vec![CONTENT_TYPE])
        .supports_credentials()
        .max_age(3600)
}
