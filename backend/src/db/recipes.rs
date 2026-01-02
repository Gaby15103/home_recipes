use std::fs;
use std::path::Path;
use super::DbExecutor;
use crate::app::recipes::{CreateRecipe, GetAllRecipes, UpdateRecipe};
use crate::db::step::{create_step_groups, fetch_step_groups_for_recipe};
use crate::db::ingredients::{create_ingredient_groups, fetch_ingredient_groups_for_recipe};
use crate::db::tags::{create_or_associate_tags, fetch_tags_for_recipe};
use crate::dto::{
    CreateRecipeInput, IngredientGroupResponse, RecipeResponse, StepGroupResponse, TagResponse,
};
use crate::models::{NewRecipe, Recipe, RecipeChange};
use crate::prelude::*;
use actix::prelude::*;
use actix_multipart::form::tempfile::TempFile;
use diesel::prelude::*;
use crate::schema::recipes::{created_at, is_private};
use crate::schema::recipes::dsl::recipes;

impl Message for CreateRecipe {
    type Result = Result<RecipeResponse>;
}

impl Handler<CreateRecipe> for DbExecutor {
    type Result = Result<RecipeResponse>;

    fn handle(&mut self, msg: CreateRecipe, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;
        use std::fs;
        use std::path::{Path, PathBuf};
        use uuid::Uuid;

        let image_dir = PathBuf::from("assets/recipes");

        fs::create_dir_all(&image_dir)
            .map_err(|e| {
                log::error!("Failed to create image directory: {}", e);
                diesel::result::Error::RollbackTransaction
            })?;

        let temp_file = msg.main_image;

        let extension = temp_file
            .file_name
            .as_deref()
            .and_then(|name| Path::new(name).extension())
            .and_then(|ext| ext.to_str())
            .unwrap_or("png");

        let file_name = format!(
            "recipe_{}_{}.{}",
            Uuid::new_v4(),
            chrono::Utc::now().timestamp(),
            extension
        );

        let disk_path = image_dir.join(&file_name);

        fs::copy(temp_file.file.path(), &disk_path)
            .map_err(|e| {
                log::error!("Failed to copy recipe image: {}", e);
                diesel::result::Error::RollbackTransaction
            })?;

        let other_image_url = format!("/assets/recipes/{}", file_name);

        let mut conn = self.0.get()?;

        let new_recipe = NewRecipe {
            title: msg.new_recipe.title,
            description: msg.new_recipe.description,
            image_url: other_image_url.to_string(),
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
        let inserted_tags = create_or_associate_tags(
            &mut conn,
            inserted_recipe.id,
            msg.new_recipe.tags,
        )?;
        let inserted_ingredient_groups = create_ingredient_groups(
            &mut conn,
            inserted_recipe.id,
            msg.new_recipe.ingredient_groups,
        )?;
        let inserted_step_groups = create_step_groups(
            &mut conn,
            inserted_recipe.id,
            msg.new_recipe.step_groups,
            msg.step_image,
        )?;

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
            image_url: "".to_string(),
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
                image_url: "".to_string(),
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

impl Message for GetAllRecipes {
    type Result = Result<Vec<RecipeResponse>>;
}

impl Handler<GetAllRecipes> for DbExecutor {
    type Result = Result<Vec<RecipeResponse>>;

    fn handle(
        &mut self,
        msg: GetAllRecipes,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::recipes::dsl::*;

        let mut conn = self.0.get()?;

        let mut query = recipes.into_boxed();

        if !msg.include_private {
            query = query.filter(is_private.eq(false));
        }

        // 🔍 Filters
        if let Some(search) = &msg.filters.search {
            let pattern = format!("%{}%", search);

            query = query.filter(
                title.ilike(pattern.clone())
                    .or(description.ilike(pattern))
            );
        }


        if let Some(min) = msg.filters.min_prep {
            query = query.filter(prep_time_minutes.ge(min));
        }

        if let Some(max) = msg.filters.max_prep {
            query = query.filter(prep_time_minutes.le(max));
        }

        if let Some(min) = msg.filters.min_cook {
            query = query.filter(cook_time_minutes.ge(min));
        }

        if let Some(max) = msg.filters.max_cook {
            query = query.filter(cook_time_minutes.le(max));
        }

        if let Some(from) = msg.filters.date_from {
            query = query.filter(created_at.ge(from.and_hms_opt(0, 0, 0).unwrap()));
        }

        if let Some(to) = msg.filters.date_to {
            query = query.filter(created_at.le(to.and_hms_opt(23, 59, 59).unwrap()));
        }

        let recipe_models: Vec<Recipe> = query
            .order(created_at.desc())
            .load(&mut conn)?;

        let mut result = Vec::with_capacity(recipe_models.len());

        for recipe in recipe_models {
            let recipe_id = recipe.id;

            let tags = fetch_tags_for_recipe(&mut conn, recipe_id)?;
            let ingredient_groups =
                fetch_ingredient_groups_for_recipe(&mut conn, recipe_id)?;
            let step_groups =
                fetch_step_groups_for_recipe(&mut conn, recipe_id)?;

            result.push(
                RecipeResponse::from_parts(
                    recipe,
                    tags,
                    ingredient_groups,
                    step_groups,
                ),
            );
        }

        Ok(result)
    }
}


