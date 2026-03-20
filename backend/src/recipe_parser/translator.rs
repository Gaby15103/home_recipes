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
        .map_err(|_| Error::InternalServerError)?;

    let body: serde_json::Value = res.json().await.map_err(|_| Error::InternalServerError)?;

    Ok(body["translatedText"]
        .as_str()
        .unwrap_or(text)
        .to_string())
}