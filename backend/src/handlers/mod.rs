use crate::{AppState, model::User, schema::UserCreateSchema};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;

pub async fn hello_world() -> impl IntoResponse {
    let json_response = json!({
        "status": "ok",
        "message": "Hello, World!"
    });
    Json(json_response)
}

pub async fn create_game_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<UserCreateSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = uuid::Uuid::new_v4();
    let user = sqlx::query_as!(
        User,
        r#"
    INSERT INTO users (
        id, email, username, password_hash, display_name, avatar_url, preferences
    )
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    RETURNING
        id,
        email,
        username,
        password_hash,
        display_name,
        avatar_url,
        preferences,
        is_active,
        email_verified,
        last_login_at,
        created_at,
        updated_at
    "#,
        id,
        body.email,
        body.username,
        body.password,
        body.display_name,
        body.avatar_url,
        body.preferences
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

    if let Err(err) = user {
        if err.to_string().contains("duplicate key value") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Game already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let user_response = json!({
        "status": "success",
        "data": json!({
            "user": user
        })
    });

    Ok(Json(user_response))
}
