use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeAnalyticsResponse {
    pub views: i64,
}

pub struct RegisterRecipeView {
    pub recipe_id: Uuid,
    pub user_id: Option<Uuid>,
}
pub struct GetRecipeAnalytics {
    pub recipe_id: Uuid,
}