use std::collections::HashMap;
use actix_ws::Session;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct NotificationHub {
    pub clients: RwLock<HashMap<Uuid, Vec<Session>>>,
}

impl NotificationHub {
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add_ws_client(&self, user_id: Uuid, session: Session) {
        let mut clients = self.clients.write().await;
        clients.entry(user_id).or_insert_with(Vec::new).push(session);
    }

    pub async fn remove_ws_client(&self, user_id: Uuid) {
        let mut clients = self.clients.write().await;
        clients.remove(&user_id);
    }

    pub async fn broadcast_to_user(&self, user_id: Uuid, payload: String) {
        let mut clients = self.clients.write().await;
        if let Some(sessions) = clients.get_mut(&user_id) {
            let mut to_remove = Vec::new();

            for (idx, session) in sessions.iter_mut().enumerate() {
                if session.text(payload.clone()).await.is_err() {
                    to_remove.push(idx);
                }
            }

            for idx in to_remove.into_iter().rev() {
                sessions.remove(idx);
            }
        }
    }
}