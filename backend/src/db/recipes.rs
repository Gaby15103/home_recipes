use super::DbExecutor;
use crate::app::recipes::{GetAllRecipes, GetAllRecipesByPage, PaginatedRecipes};
use crate::db::ingredients::{
    create_ingredient_groups, fetch_ingredient_groups_for_recipe, sync_ingredient_groups,
};
use crate::db::step::{create_step_groups, fetch_step_groups_for_recipe, sync_step_groups};
use crate::db::tags::{create_or_associate_tags, fetch_tags_for_recipe};
use crate::dto::{CreateComment, CreateRecipe, DeleteComment, DeleteRecipe, GetFavoriteRecipes, GetRecipeAnalytics, GetRecipeById, GetRecipeComments, GetRecipeRating, GetRecipeVersion, GetRecipeVersions, IngredientGroupResponse, RecipeCommentResponse, RecipeRatingResponse, RecipeResponse, RecipeVersionResponse, RegisterRecipeView, RestoreRecipeVersion, SetRecipeRating, StepGroupResponse, TagResponse, ToggleFavorite, UnsetRecipeRating, UpdateRecipe, UpdateRecipeInput};
use crate::models::{NewFavorite, NewRecipe, NewRecipeAnalytics, NewRecipeComment, NewRecipeRating, NewRecipeVersion, Recipe, RecipeChange, RecipeComment, RecipeVersion, User};
use crate::prelude::*;
use crate::utils::image_upload::upload_recipe_image;
use actix::prelude::*;
use actix_web::error::ErrorNotFound;
use chrono::Utc;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::{Double, Nullable};
use uuid::Uuid;
use crate::error::DbError;

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

        let current_recipe_snapshot = RecipeResponse::from_parts(
            recipes.find(msg.update_recipe.id).first::<Recipe>(&mut conn)?,
            fetch_tags_for_recipe(&mut conn, msg.update_recipe.id)?,
            fetch_ingredient_groups_for_recipe(&mut conn, msg.update_recipe.id)?,
            fetch_step_groups_for_recipe(&mut conn, msg.update_recipe.id)?,
        );

        create_recipe_version(&mut conn, &current_recipe_snapshot, msg.auth.user)?;


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

fn create_recipe_version(
    conn: &mut PgConnection,
    recipe_snapshot: &RecipeResponse, // full current recipe
    user: User,
) -> Result<Uuid> {
    use crate::schema::recipe_versions::dsl::*;

    let snapshot_json = serde_json::to_value(recipe_snapshot).unwrap();

    let new_version = NewRecipeVersion {
        recipe_id: recipe_snapshot.id,
        data: snapshot_json,
        edited_by: user.id,
    };

    // Insert and get the generated id
    let inserted_id: Uuid = diesel::insert_into(recipe_versions)
        .values(&new_version)
        .returning(crate::schema::recipe_versions::id)
        .get_result(conn)?;

    Ok(inserted_id)
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

impl Message for DeleteRecipe {
    type Result = Result<(), DbError>;
}

impl Handler<DeleteRecipe> for DbExecutor {
    type Result = Result<(), DbError>;

    fn handle(&mut self, msg: DeleteRecipe, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;

        let mut conn = self.0.get().map_err(DbError::Pool)?;

        let affected = diesel::delete(recipes.filter(id.eq(msg.recipe_id)))
            .execute(&mut conn)?;

        if affected == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }
}



impl Message for ToggleFavorite {
    type Result = Result<bool>;
}

impl Handler<ToggleFavorite> for DbExecutor {
    type Result = Result<bool>;

    fn handle(&mut self, msg: ToggleFavorite, _: &mut Self::Context) -> Self::Result {
        use crate::schema::favorites::dsl::*;

        let mut conn = self.0.get()?;

        let exists = diesel::select(diesel::dsl::exists(
            favorites
                .filter(user_id.eq(msg.user_id))
                .filter(recipe_id.eq(msg.recipe_id)),
        ))
            .get_result::<bool>(&mut conn)?;

        if exists {
            diesel::delete(
                favorites
                    .filter(user_id.eq(msg.user_id))
                    .filter(recipe_id.eq(msg.recipe_id)),
            )
                .execute(&mut conn)?;
            Ok(false)
        } else {
            diesel::insert_into(favorites)
                .values(NewFavorite {
                    user_id: msg.user_id,
                    recipe_id: msg.recipe_id,
                })
                .execute(&mut conn)?;
            Ok(true)
        }
    }
}

impl Message for GetFavoriteRecipes {
    type Result = Result<Vec<RecipeResponse>>;
}

impl Handler<GetFavoriteRecipes> for DbExecutor {
    type Result = Result<Vec<RecipeResponse>>;

    fn handle(&mut self, msg: GetFavoriteRecipes, _: &mut Self::Context) -> Self::Result {
        use crate::schema::{favorites, recipes};

        let mut conn = self.0.get()?;

        // 1) Load all favorited recipes for user
        let recipe_rows: Vec<Recipe> = favorites::table
            .inner_join(recipes::table)
            .filter(favorites::user_id.eq(msg.user_id))
            .select(recipes::all_columns)
            .order(favorites::created_at.desc())
            .load(&mut conn)?;

        // 2) Build full responses
        let mut responses = Vec::with_capacity(recipe_rows.len());

        for recipe in recipe_rows {
            let recipe_id = recipe.id;

            let tags = fetch_tags_for_recipe(&mut conn, recipe_id)?;
            let ingredient_groups = fetch_ingredient_groups_for_recipe(&mut conn, recipe_id)?;
            let step_groups = fetch_step_groups_for_recipe(&mut conn, recipe_id)?;

            responses.push(
                RecipeResponse::from_parts(
                    recipe,
                    tags,
                    ingredient_groups,
                    step_groups,
                )
            );
        }

        Ok(responses)
    }
}


impl Message for SetRecipeRating {
    type Result = Result<()>;
}
impl Handler<SetRecipeRating> for DbExecutor {
    type Result = Result<()>;

    fn handle(&mut self, msg: SetRecipeRating, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipe_ratings::dsl::*;

        let mut conn = self.0.get()?;

        diesel::insert_into(recipe_ratings)
            .values(NewRecipeRating {
                recipe_id: msg.recipe_id,
                user_id: msg.user_id,
                rating: msg.rating,
            })
            .on_conflict((recipe_id, user_id))
            .do_update()
            .set(rating.eq(msg.rating))
            .execute(&mut conn)?;

        Ok(())
    }
}

impl Message for GetRecipeRating {
    type Result = Result<RecipeRatingResponse>;
}

impl Handler<GetRecipeRating> for DbExecutor {
    type Result = Result<RecipeRatingResponse>;

    fn handle(&mut self, msg: GetRecipeRating, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipe_ratings::dsl::*;

        let mut conn = self.0.get()?;

        let (avg, count): (Option<f64>, i64) = recipe_ratings
            .filter(recipe_id.eq(msg.recipe_id))
            .select((
                sql::<Nullable<Double>>("AVG(rating)"),
                diesel::dsl::count_star(),
            ))
            .first(&mut conn)?;

        let user_rating = if let Some(uid) = msg.user_id {
            recipe_ratings
                .filter(recipe_id.eq(msg.recipe_id))
                .filter(user_id.eq(uid))
                .select(rating)
                .first(&mut conn)
                .optional()?
        } else {
            None
        };

        Ok(RecipeRatingResponse {
            average: avg.unwrap_or(0.0) as f32,
            count,
            user_rating,
        })
    }
}

impl Message for UnsetRecipeRating {
    type Result = Result<()>;
}

impl Handler<UnsetRecipeRating> for DbExecutor {
    type Result = Result<()>;

    fn handle(&mut self, msg: UnsetRecipeRating, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipe_ratings::dsl::*;

        let mut conn = self.0.get()?;

        diesel::delete(
            recipe_ratings
                .filter(recipe_id.eq(msg.recipe_id))
                .filter(user_id.eq(msg.user_id)),
        )
            .execute(&mut conn)?;

        Ok(())
    }
}


impl Message for CreateComment {
    type Result = Result<RecipeCommentResponse>;
}

impl Handler<CreateComment> for DbExecutor {
    type Result = Result<RecipeCommentResponse>;

    fn handle(&mut self, msg: CreateComment, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipe_comments::dsl::*;

        let mut conn = self.0.get()?;

        let comment = diesel::insert_into(recipe_comments)
            .values(NewRecipeComment {
                recipe_id: msg.recipe_id,
                user_id: msg.user_id,
                parent_id: msg.parent_id,
                content: msg.content,
            })
            .get_result::<RecipeComment>(&mut conn)?;

        Ok(RecipeCommentResponse {
            id: comment.id,
            user_id: comment.user_id,
            content: comment.content,
            created_at: comment.created_at,
            edited_at: comment.edited_at,
            children: Vec::new(),
        })
    }
}

impl Message for GetRecipeComments {
    type Result = Result<Vec<RecipeCommentResponse>>;
}

impl Handler<GetRecipeComments> for DbExecutor {
    type Result = Result<Vec<RecipeCommentResponse>>;

    fn handle(&mut self, msg: GetRecipeComments, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipe_comments::dsl::*;

        let mut conn = self.0.get()?;

        let rows: Vec<RecipeComment> = recipe_comments
            .filter(recipe_id.eq(msg.recipe_id))
            .filter(deleted_at.is_null())
            .order(created_at.asc())
            .load(&mut conn)?;

        // map id -> response node
        use std::collections::HashMap;
        let mut map: HashMap<Uuid, RecipeCommentResponse> = HashMap::new();

        for c in &rows {
            map.insert(
                c.id,
                RecipeCommentResponse {
                    id: c.id,
                    user_id: c.user_id,
                    content: c.content.clone(),
                    created_at: c.created_at,
                    edited_at: c.edited_at,
                    children: Vec::new(),
                },
            );
        }

        let mut roots = Vec::new();

        for c in rows {
            let node = map.remove(&c.id).unwrap();

            if let Some(pid) = c.parent_id {
                if let Some(parent) = map.get_mut(&pid) {
                    parent.children.push(node);
                }
            } else {
                roots.push(node);
            }
        }
        Ok(roots)
    }
}


impl Message for DeleteComment {
    type Result = Result<()>;
}

impl Handler<DeleteComment> for DbExecutor {
    type Result = Result<()>;

    fn handle(&mut self, msg: DeleteComment, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipe_comments::dsl::*;

        let mut conn = self.0.get()?;

        diesel::update(recipe_comments.find(msg.comment_id))
            .set(deleted_at.eq(Some(Utc::now())))
            .execute(&mut conn)?;

        Ok(())
    }
}

impl Message for RegisterRecipeView {
    type Result = Result<()>;
}
impl Handler<RegisterRecipeView> for DbExecutor {
    type Result = Result<()>;

    fn handle(&mut self, msg: RegisterRecipeView, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipe_analytics::dsl::*;

        let mut conn = self.0.get()?;

        diesel::insert_into(recipe_analytics)
            .values(NewRecipeAnalytics {
                recipe_id: msg.recipe_id,
                user_id: msg.user_id,
            })
            .execute(&mut conn)?;

        Ok(())
    }
}

impl Message for GetRecipeAnalytics {
    type Result = Result<i64>;
}

impl Handler<GetRecipeAnalytics> for DbExecutor {
    type Result = Result<i64>;

    fn handle(&mut self, msg: GetRecipeAnalytics, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipe_analytics::dsl::*;

        let mut conn = self.0.get()?;

        let count = recipe_analytics
            .filter(recipe_id.eq(msg.recipe_id))
            .count()
            .get_result(&mut conn)?;

        Ok(count)
    }
}

impl Message for GetRecipeVersions {
    type Result = Result<Vec<RecipeVersionResponse>>;
}

impl Handler<GetRecipeVersions> for DbExecutor {
    type Result = Result<Vec<RecipeVersionResponse>, Error>;

    fn handle(&mut self, msg: GetRecipeVersions, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipe_versions::dsl::*;

        let mut conn = self.0.get()?;

        // Load all versions for this recipe directly as RecipeVersion structs
        let versions: Vec<RecipeVersion> = recipe_versions
            .filter(recipe_id.eq(msg.recipe_id))
            .order(created_at.desc())
            .load::<RecipeVersion>(&mut conn)?;

        // Convert to your response type, deserializing JSONB
        let responses: Vec<RecipeVersionResponse> = versions
            .into_iter()
            .map(|v| {
                let recipe: RecipeResponse = serde_json::from_value(v.data)
                    .expect("Failed to deserialize recipe JSON");

                RecipeVersionResponse {
                    id: v.id,
                    recipe_id: v.recipe_id,
                    recipe,
                    created_at: v.created_at,
                    edited_by: v.edited_by,
                }
            })
            .collect();

        Ok(responses)
    }
}

impl Message for GetRecipeVersion {
    type Result = Result<RecipeVersionResponse>;
}

impl Handler<GetRecipeVersion> for DbExecutor {
    type Result = Result<RecipeVersionResponse>;

    fn handle(&mut self, msg: GetRecipeVersion, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipe_versions::dsl::*;

        let mut conn = self.0.get()?;

        let row: (Uuid, Uuid, serde_json::Value, chrono::NaiveDateTime, Uuid) =
            recipe_versions
                .filter(id.eq(msg.id))
                .select((id, recipe_id, data, created_at, edited_by))
                .first(&mut conn)?;

        let recipe: RecipeResponse = serde_json::from_value(row.2)
            .expect("Failed to deserialize recipe JSON for version");

        Ok(RecipeVersionResponse {
            id: row.0,
            recipe_id: row.1,
            recipe,
            created_at: chrono::DateTime::<chrono::Utc>::from_utc(row.3, chrono::Utc),
            edited_by: row.4,
        })
    }
}

impl Message for RestoreRecipeVersion {
    type Result = Result<RecipeResponse, Error>;
}

impl Handler<RestoreRecipeVersion> for DbExecutor {
    type Result = Result<RecipeResponse>;

    fn handle(&mut self, msg: RestoreRecipeVersion, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;
        use crate::schema::recipe_versions::dsl as versions_dsl;

        let mut conn = self.0.get()?;

        // Fetch version JSON
        let row: (Uuid, Uuid, serde_json::Value, chrono::NaiveDateTime, Uuid) =
            versions_dsl::recipe_versions
                .filter(versions_dsl::id.eq(msg.version_id))
                .select((versions_dsl::id, versions_dsl::recipe_id, versions_dsl::data, versions_dsl::created_at, versions_dsl::edited_by))
                .first(&mut conn)?;

        let version_recipe: Recipe = serde_json::from_value(row.2)
            .expect("Failed to deserialize recipe JSON for restoration");

        // Update main recipe
        let restored_recipe: Recipe = diesel::update(recipes.find(row.1))
            .set((
                title.eq(&version_recipe.title),
                description.eq(&version_recipe.description),
                image_url.eq(&version_recipe.image_url),
                servings.eq(version_recipe.servings),
                prep_time_minutes.eq(version_recipe.prep_time_minutes),
                cook_time_minutes.eq(version_recipe.cook_time_minutes),
                author.eq(&version_recipe.author),
                author_id.eq(version_recipe.author_id),
                is_private.eq(version_recipe.is_private),
            ))
            .get_result(&mut conn)?;

        // You could optionally restore tags, steps, ingredients here if needed

        // Return restored recipe
        Ok(RecipeResponse::from_parts(
            restored_recipe,
            fetch_tags_for_recipe(&mut conn, row.1)?,
            fetch_ingredient_groups_for_recipe(&mut conn, row.1)?,
            fetch_step_groups_for_recipe(&mut conn, row.1)?,
        ))
    }
}
