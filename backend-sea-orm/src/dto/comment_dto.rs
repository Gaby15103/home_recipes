use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use entity::recipe_comments;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CommentDto {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateCommentDto {
    pub recipe_id: Uuid,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
}
impl From<recipe_comments::Model> for CommentDto {
    fn from(value: recipe_comments::Model) -> Self {
        let display_content = if value.deleted_at.is_some() {
            "This comment has been deleted.".to_string()
        } else {
            value.content
        };
        Self {
            id: value.id,
            recipe_id: value.recipe_id,
            user_id: value.user_id.unwrap_or_else(Uuid::nil),
            parent_id: value.parent_id,
            content: display_content,
            created_at: value.created_at.with_timezone(&Utc),
            edited_at: value.edited_at
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|| value.created_at.with_timezone(&Utc)),
            deleted_at: value.deleted_at.map(|dt| dt.with_timezone(&Utc)),
        }
    }
}