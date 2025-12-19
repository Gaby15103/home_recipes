use diesel::{Identifiable, Queryable, Associations, Insertable};
use uuid::Uuid;
use crate::schema::recipe_tags;
use crate::models::{recipe::Recipe, tag::Tag};

#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(
    table_name = recipe_tags,
    primary_key(recipe_id, tag_id),
    belongs_to(Recipe),
    belongs_to(Tag)
)]
pub struct RecipeTag {
    pub recipe_id: Uuid,
    pub tag_id: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = recipe_tags)]
pub struct NewRecipeTag {
    pub recipe_id: Uuid,
    pub tag_id: Uuid,
}
