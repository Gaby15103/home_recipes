use actix::prelude::*;
use diesel::prelude::*;

use super::DbExecutor;
use crate::app::recipes::{CreateRecipeOuter, RecipeResponse, UpdateRecipeOuter};
use crate::models::{NewRecipe, Recipe, RecipeChange, Tag, User, UserChange};
use crate::prelude::*;
use crate::schema::tags::dsl::tags;

impl Message for CreateRecipeOuter {
    type Result = Result<RecipeResponse>;
}

impl Handler<CreateRecipeOuter> for DbExecutor{
    type Result = Result<RecipeResponse>;

    fn handle(&mut self, msg: CreateRecipeOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;

        let mut conn = self.0.get()?;

        let new_recipe = NewRecipe{
            title: msg.new_recipe.title,
            description: msg.new_recipe.description,
            servings: msg.new_recipe.servings,
            prep_time_minutes: msg.new_recipe.prep_time_minutes,
            cook_time_minutes: msg.new_recipe.cook_time_minutes,
            author: msg.new_recipe.author,
            author_id: msg.new_recipe.author_id,
            is_private: msg.new_recipe.is_private,
        };

        let inserted_recipe: Recipe = diesel::insert_into(recipes)
            .values(&new_recipe)
            .get_result(&mut conn)?;

        Ok(RecipeResponse::from(inserted_recipe))
    }
}

impl Message for UpdateRecipeOuter {
    type Result = Result<RecipeResponse>;
}

impl Handler<UpdateRecipeOuter> for DbExecutor{
    type Result = Result<RecipeResponse>;

    fn handle(&mut self, msg: UpdateRecipeOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;

        let mut conn = self.0.get()?;

        let update_recipe = RecipeChange{
            title: msg.update_recipe.title,
            description: msg.update_recipe.description,
            servings: msg.update_recipe.servings,
            prep_time_minutes: msg.update_recipe.prep_time_minutes,
            cook_time_minutes: msg.update_recipe.cook_time_minutes,
            author: msg.update_recipe.author,
            author_id: msg.update_recipe.author_id,
            is_private: msg.update_recipe.is_private,
        };

        match diesel::update(recipes.find(msg.update_recipe.id))
            .set(&update_recipe)
            .get_result::<Recipe>(&mut conn)
        {
            Ok(recipe) => Ok(recipe.into()),
            Err(e) => Err(e.into()),
        }
    }
}