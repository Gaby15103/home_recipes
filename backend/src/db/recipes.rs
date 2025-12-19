use super::DbExecutor;
use crate::app::recipes::{CreateRecipe, UpdateRecipe};
use crate::app::step::create_step_groups;
use crate::db::ingredients::create_ingredient_groups;
use crate::db::tags::create_or_associate_tags;
use crate::dto::{
    CreateRecipeInput, IngredientGroupResponse, RecipeResponse, StepGroupResponse, TagResponse,
};
use crate::models::{NewRecipe, Recipe, RecipeChange};
use crate::prelude::*;
use actix::prelude::*;
use diesel::prelude::*;

impl Message for CreateRecipe {
    type Result = Result<RecipeResponse>;
}

impl Handler<CreateRecipe> for DbExecutor {
    type Result = Result<RecipeResponse>;

    fn handle(&mut self, msg: CreateRecipe, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;

        let mut conn = self.0.get()?;

        let new_recipe = NewRecipe {
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

        let inserted_tags: Vec<TagResponse> =
            create_or_associate_tags(&mut conn, inserted_recipe.id, msg.new_recipe.tags)?;

        let inserted_ingredient_groups: Vec<IngredientGroupResponse> = create_ingredient_groups(
            &mut conn,
            inserted_recipe.id,
            msg.new_recipe.ingredient_groups,
        )?;

        let inserted_step_groups: Vec<StepGroupResponse> =
            create_step_groups(&mut conn, inserted_recipe.id, msg.new_recipe.step_groups)?;

        Ok(RecipeResponse::from_parts(
            inserted_recipe,
            inserted_tags,
            inserted_ingredient_groups,
            inserted_step_groups,
        ))
    }
}

impl Message for UpdateRecipe {
    type Result = Result<RecipeResponse>;
}

impl Handler<UpdateRecipe> for DbExecutor {
    type Result = Result<RecipeResponse>;

    fn handle(&mut self, msg: UpdateRecipe, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;

        let mut conn = self.0.get()?;

        let update_recipe = RecipeChange {
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
            Ok(recipe) => Ok(RecipeResponse {
                id: recipe.id,
                title: recipe.title,
                description: recipe.description,
                servings: recipe.servings,
                prep_time_minutes: recipe.cook_time_minutes,
                cook_time_minutes: recipe.cook_time_minutes,
                author: recipe.author,
                author_id: recipe.author_id,
                is_private: recipe.is_private,
                tags: vec![],
                ingredient_groups: vec![],
                step_groups: vec![],
            }),
            Err(e) => Err(e.into()),
        }
    }
}
