use crate::app::state::AppState;
use crate::domain::user::{AuthenticatedUser, Role};
use crate::dto::notification_dto::{CreateNotificationTemplateInput, NotificationListResponse};
use crate::errors::Error;
use actix_web::web::{Data, Json, Path, Payload};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_ws::Message;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::services::notification_service;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notifications")
            // WebSocket endpoint for real-time updates
            .route("/ws", web::get().to(notification_ws))
            // Inbox management
            .route("", web::get().to(list))
            .route("/{id}/read", web::post().to(mark_as_read))
            .route("/read-all", web::post().to(mark_all_as_read))
            // Template management (Admin/Mod only)
            .route("/templates", web::post().to(create_template))
    );
}

pub async fn notification_ws(
    req: HttpRequest,
    body: Payload,
    state: Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, actix_web::Error> {
    let (res, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    let user_id = auth.user.id;
    let hub = state.notification_hub.clone();

    // Spawn a task to handle the WebSocket lifecycle
    actix_web::rt::spawn(async move {
        // Register connection in the hub
        hub.add_ws_client(user_id, session.clone()).await;

        // Keep the connection alive and handle client messages
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        break;
                    }
                }
                Message::Text(_) => {
                    // Optional: Handle incoming messages from the frontend
                }
                Message::Close(reason) => {
                    let _ = session.close(reason).await;
                    break;
                }
                _ => (),
            }
        }

        // Cleanup on disconnect
        hub.remove_ws_client(user_id).await;
    });

    Ok(res)
}

pub async fn list(
    state: Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    let notifications = notification_service::get_for_user(&state.db, auth.user.id).await?;
    Ok(HttpResponse::Ok().json(notifications))
}

pub async fn mark_as_read(
    state: Data<AppState>,
    path: Path<Uuid>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    notification_service::mark_as_read(&state.db, path.into_inner(), auth.user.id).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn mark_all_as_read(
    state: Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    notification_service::mark_all_read(&state.db, auth.user.id).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn create_template(
    state: Data<AppState>,
    auth: AuthenticatedUser,
    input: Json<CreateNotificationTemplateInput>,
) -> Result<HttpResponse, Error> {
    auth.require_roles(&[Role::Admin, Role::Moderator, Role::Superuser])?;

    let template = notification_service::create_template(&state.db, input.into_inner()).await?;
    Ok(HttpResponse::Created().json(template))
}