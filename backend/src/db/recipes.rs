use super::DbExecutor;
use crate::app::recipes::{
    CreateRecipe, GetAllRecipes, GetAllRecipesByPage, GetRecipeById, PaginatedRecipes, UpdateRecipe,
};
use crate::db::ingredients::{
    create_ingredient_groups, fetch_ingredient_groups_for_recipe, sync_ingredient_groups,
};
use crate::db::step::{create_step_groups, fetch_step_groups_for_recipe, sync_step_groups};
use crate::db::tags::{create_or_associate_tags, fetch_tags_for_recipe};
use crate::dto::{
    CreateRecipeInput, IngredientGroupResponse, RecipeResponse, StepGroupResponse, TagResponse,
};
use crate::models::{IngredientGroup, NewRecipe, Recipe, RecipeChange};
use crate::prelude::*;
use crate::schema::recipes::dsl::recipes;
use crate::schema::recipes::{created_at, is_private};
use crate::utils::image_upload::upload_recipe_image;
use actix::prelude::*;
use actix_multipart::form::tempfile::TempFile;
use diesel::prelude::*;
use std::fs;
use std::path::Path;

impl Message for GetRecipeById {
    type Result = Result<RecipeResponse>;
}

impl Handler<GetRecipeById> for DbExecutor {
    type Result = Result<RecipeResponse>;

    fn handle(&mut self, msg: GetRecipeById, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;

        let mut conn = self.0.get()?;

        // Fetch the main recipe
        let recipe_model: Recipe = recipes.find(msg.id).first(&mut conn)?;

        // Fetch related data
        let tags = fetch_tags_for_recipe(&mut conn, recipe_model.id)?;
        let ingredient_groups = fetch_ingredient_groups_for_recipe(&mut conn, recipe_model.id)?;
        let step_groups = fetch_step_groups_for_recipe(&mut conn, recipe_model.id)?;

        // Compose response
        Ok(RecipeResponse::from_parts(
            recipe_model,
            tags,
            ingredient_groups,
            step_groups,
        ))
    }
}

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

        fs::create_dir_all(&image_dir).map_err(|e| {
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

        fs::copy(temp_file.file.path(), &disk_path).map_err(|e| {
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
        let inserted_tags =
            create_or_associate_tags(&mut conn, inserted_recipe.id, msg.new_recipe.tags)?;
        let inserted_ingredient_groups = create_ingredient_groups(
            &mut conn,
            inserted_recipe.id,
            msg.new_recipe.ingredient_groups,
        )?;
        let inserted_step_groups = create_step_groups(
            &mut conn,
            inserted_recipe.id,
            msg.new_recipe.step_groups,
            msg.step_images,
            msg.step_images_meta,
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

        let new_image_url: String = if let Some(temp_file) = msg.main_image {
            upload_recipe_image(temp_file)?
        } else {
            // fallback to existing image_url, or empty string if None
            msg.update_recipe.image_url.clone().unwrap_or_default()
        };

        let change = RecipeChange {
            title: msg.update_recipe.title,
            description: msg.update_recipe.description,
            image_url: new_image_url,
            servings: msg.update_recipe.servings,
            prep_time_minutes: msg.update_recipe.prep_time_minutes,
            cook_time_minutes: msg.update_recipe.cook_time_minutes,
            author: msg.update_recipe.author,
            author_id: msg.update_recipe.author_id,
            is_private: msg.update_recipe.is_private,
        };

        let recipe: Recipe = diesel::update(recipes.find(msg.update_recipe.id))
            .set(change)
            .get_result(&mut conn)?;

        let tags = create_or_associate_tags(&mut conn, recipe.id, msg.update_recipe.tags)?;

        let ingredient_groups =
            sync_ingredient_groups(&mut conn, recipe.id, msg.update_recipe.ingredient_groups)?;

        let step_groups = sync_step_groups(
            &mut conn,
            recipe.id,
            msg.update_recipe.step_groups,
            msg.step_images,
            msg.step_images_meta,
        )?;

        Ok(RecipeResponse::from_parts(
            recipe,
            tags,
            ingredient_groups,
            step_groups,
        ))
    }
}

impl Message for GetAllRecipes {
    type Result = Result<Vec<RecipeResponse>>;
}

impl Handler<GetAllRecipes> for DbExecutor {
    type Result = Result<Vec<RecipeResponse>>;

    fn handle(&mut self, msg: GetAllRecipes, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;

        let mut conn = self.0.get()?;

        let mut query = recipes.into_boxed();

        if !msg.include_private {
            query = query.filter(is_private.eq(false));
        }

        // 🔍 Filters
        if let Some(search) = &msg.filters.search {
            let pattern = format!("%{}%", search);

            query = query.filter(title.ilike(pattern.clone()).or(description.ilike(pattern)));
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

        let recipe_models: Vec<Recipe> = query.order(created_at.desc()).load(&mut conn)?;

        let mut result = Vec::with_capacity(recipe_models.len());

        for recipe in recipe_models {
            let recipe_id = recipe.id;

            let tags = fetch_tags_for_recipe(&mut conn, recipe_id)?;
            let ingredient_groups = Vec::new();
            //fetch_ingredient_groups_for_recipe(&mut conn, recipe_id)?;
            let step_groups = Vec::new();
            //fetch_step_groups_for_recipe(&mut conn, recipe_id)?;

            result.push(RecipeResponse::from_parts(
                recipe,
                tags,
                ingredient_groups,
                step_groups,
            ));
        }

        Ok(result)
    }
}

impl Message for GetAllRecipesByPage {
    type Result = Result<PaginatedRecipes>;
}

impl Handler<GetAllRecipesByPage> for DbExecutor {
    type Result = Result<PaginatedRecipes>;

    fn handle(&mut self, msg: GetAllRecipesByPage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;

        let mut conn = self.0.get()?;

        let filters = msg.filters.as_ref();
        let include_private = msg.include_private;

        let build_query = || {
            use crate::schema::recipes::dsl::*;

            let mut q = recipes.into_boxed();

            if !include_private {
                q = q.filter(is_private.eq(false));
            }

            if let Some(filters) = filters {
                if let Some(search) = &filters.search {
                    let pattern = format!("%{}%", search);
                    q = q.filter(title.ilike(pattern.clone()).or(description.ilike(pattern)));
                }

                if let Some(min) = filters.min_prep {
                    q = q.filter(prep_time_minutes.ge(min));
                }
                if let Some(max) = filters.max_prep {
                    q = q.filter(prep_time_minutes.le(max));
                }

                if let Some(min) = filters.min_cook {
                    q = q.filter(cook_time_minutes.ge(min));
                }
                if let Some(max) = filters.max_cook {
                    q = q.filter(cook_time_minutes.le(max));
                }

                if let Some(from) = filters.date_from {
                    q = q.filter(created_at.ge(from.and_hms_opt(0, 0, 0).unwrap()));
                }
                if let Some(to) = filters.date_to {
                    q = q.filter(created_at.le(to.and_hms_opt(23, 59, 59).unwrap()));
                }
            }

            q
        };

        // Total count
        let total: i64 = build_query().count().get_result(&mut conn)?;

        // Pagination
        let page = msg.page.unwrap_or(1).max(1);
        let per_page = msg.per_page.unwrap_or(10).max(1);
        let offset = (page - 1) * per_page;

        // Fetch page
        let recipe_models: Vec<Recipe> = build_query()
            .order(created_at.desc())
            .limit(per_page)
            .offset(offset)
            .load(&mut conn)?;

        let mut result = Vec::with_capacity(recipe_models.len());

        for recipe in recipe_models {
            let recipe_id = recipe.id;
            let tags = fetch_tags_for_recipe(&mut conn, recipe_id)?;
            let ingredient_groups = Vec::new(); // fetch_ingredient_groups_for_recipe(&mut conn, recipe_id)?;
            let step_groups = Vec::new(); // fetch_step_groups_for_recipe(&mut conn, recipe_id)?;

            result.push(RecipeResponse::from_parts(
                recipe,
                tags,
                ingredient_groups,
                step_groups,
            ));
        }

        Ok(PaginatedRecipes {
            data: result,
            total,
            page,
            per_page,
        })
    }
}
