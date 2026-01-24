use chrono::DateTime;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeCommentResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub content: String,
    pub created_at: DateTime<chrono::Utc>,
    pub edited_at: Option<DateTime<chrono::Utc>>,
    pub children: Vec<RecipeCommentResponse>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComment {
    pub recipe_id: Uuid,
    pub user_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub content: String,
}
pub struct DeleteComment {
    pub comment_id: Uuid,
}
pub struct GetRecipeComments {
    pub recipe_id: Uuid,
}