use serde_json::json;
use crate::errors::Error;

pub async fn translate_text(text: &str, from: &str, to: &str, url: &str) -> Result<String, Error> {
    if text.trim().is_empty() || from == to {
        return Ok(text.to_string());
    }

    let client = reqwest::Client::new();
    // Use your LibreTranslate instance URL (default is usually port 5000)

    let res = client.post(url)
        .json(&json!({
            "q": text,
            "source": from,
            "target": to,
            "format": "text"
        }))
        .send()
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Translation service request failed",
            "operation": "translate_text",
            "source_lang": from,
            "target_lang": to,
            "text_length": text.len(),
            "error": e.to_string(),
            "stage": "http_request"
        })))?;

        let status = res.status();
    let raw_text = res.text().await.map_err(|e| {
        Error::InternalServerError(json!({
            "message": "Failed to read response body",
            "stage": "body_extraction",
            "error": e.to_string()
        }))
    })?;
    
    let body: serde_json::Value = serde_json::from_str(&raw_text).map_err(|e| {
        Error::InternalServerError(json!({
            "message": "Failed to parse translation response",
            "operation": "translate_text",
            "source_lang": from,
            "target_lang": to,
            "status_code": status.as_u16(),
            "raw_response": raw_text,
            "error": e.to_string(),
            "stage": "response_parsing"
        }))
    })?;

    Ok(body["translatedText"]
        .as_str()
        .unwrap_or(text)
        .to_string())
}