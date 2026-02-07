use actix_web::HttpRequest;

pub fn extract_language(req: &HttpRequest) -> String {
    if let Some(cookie) = req.cookie("lang") {
        return normalize_lang(cookie.value());
    }

    if let Some(header) = req.headers().get("Accept-Language") {
        if let Ok(lang) = header.to_str() {
            return normalize_lang(lang);
        }
    }

    "fr".to_string()
}

fn normalize_lang(header: &str) -> String {
    let lang = header
        .split(',')
        .next()
        .unwrap_or("fr")
        .split('-')
        .next()
        .unwrap_or("fr");

    match lang {
        "fr" => "fr".into(),
        "en" => "en".into(),
        _ => "fr".into(),
    }
}

