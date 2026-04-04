use serde_json::json;
use crate::errors::Error;

pub async fn translate_text(text: &str, from: &str, to: &str) -> Result<String, Error> {
    if text.trim().is_empty() || from == to {
        return Ok(text.to_string());
    }

    let client = reqwest::Client::new();
    // Use your LibreTranslate instance URL (default is usually port 5000)
    let url = "http://localhost:5000/translate";

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

    let body: serde_json::Value = res.json().await.map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to parse translation response",
        "operation": "translate_text",
        "source_lang": from,
        "target_lang": to,
        "error": e.to_string(),
        "stage": "response_parsing"
    })))?;

    Ok(body["translatedText"]
        .as_str()
        .unwrap_or(text)
        .to_string())
}