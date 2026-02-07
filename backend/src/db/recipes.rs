use std::collections::HashMap;
use std::path::Path;
use super::DbExecutor;
use crate::app::recipes::{GetAllRecipes, GetAllRecipesByPage, PaginatedRecipes};
use crate::db::ingredients::{
    create_ingredient_groups, fetch_ingredient_groups_for_recipe, sync_ingredient_groups,
};
use crate::db::step::{create_step_groups, fetch_step_groups_for_recipe, sync_step_groups};
use crate::db::tags::{create_or_associate_tags, fetch_tags_for_recipe};
use crate::dto::{CreateComment, CreateRecipe, DeleteComment, DeleteRecipe, GetFavoriteRecipes, GetRecipeAnalytics, GetRecipeById, GetRecipeComments, GetRecipeRating, GetRecipeVersion, GetRecipeVersions, IngredientGroupResponse, RecipeCommentResponse, RecipeRatingResponse, RecipeResponse, RecipeTranslationResponse, RecipeVersionResponse, RegisterRecipeView, RestoreRecipeVersion, SetRecipeRating, StepGroupResponse, TagResponse, ToggleFavorite, UnsetRecipeRating, UpdateRecipe, UpdateRecipeInput};
use crate::models::{NewFavorite, NewRecipe, NewRecipeAnalytics, NewRecipeComment, NewRecipeRating, NewRecipeVersion, Recipe, RecipeChange, RecipeComment, RecipeTranslation, RecipeVersion, User};
use crate::prelude::*;
use crate::utils::image_upload::upload_recipe_image;
use actix::prelude::*;
use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use chrono::Utc;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::{Double, Nullable};
use uuid::Uuid;
use crate::error::DbError;
use crate::schema::users;
use crate::schema::users::username;

impl Message for GetRecipeById {
    type Result = Result<RecipeResponse>;
}

impl Handler<GetRecipeById> for DbExecutor {
    type Result = Result<RecipeResponse>;

    fn handle(&mut self, msg: GetRecipeById, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;
        use crate::schema::recipe_translations::dsl as tr;

        let mut conn = self.0.get()?;

        let recipe_model: Recipe = recipes
            .filter(id.eq(msg.id))
            .first(&mut conn)?;

        let translations = tr::recipe_translations
            .filter(tr::recipe_id.eq(recipe_model.id))
            .load::<RecipeTranslation>(&mut conn)?;

        // Decide whether to return all or just one
        let translation_dtos: Vec<RecipeTranslationResponse> = if msg.include_all_translations {
            translations
                .into_iter()
                .map(|t| RecipeTranslationResponse {
                    language_code: t.language_code,
                    title: t.title,
                    description: t.description,
                })
                .collect()
        } else {
            pick_translation(&*translations, &msg.language_code, &recipe_model.original_language_code)
                .map(|t| RecipeTranslationResponse {
                    language_code: t.language_code,
                    title: t.title,
                    description: t.description,
                })
                .into_iter()
                .collect()
        };



        // Fetch related data in the requested language
        let tags = fetch_tags_for_recipe(&mut conn, recipe_model.id)?;
        let ingredient_groups =
            fetch_ingredient_groups_for_recipe(
                &mut conn,
                recipe_model.id,
                Some(&msg.language_code),
                &*recipe_model.original_language_code
            )?;
        let step_groups =
            fetch_step_groups_for_recipe(
                &mut conn,
                recipe_model.id,
                Some(&msg.language_code),
                &*recipe_model.original_language_code
            )?;

        Ok(RecipeResponse {
            id: recipe_model.id,
            image_url: recipe_model.image_url,
            servings: recipe_model.servings,
            prep_time_minutes: recipe_model.prep_time_minutes,
            cook_time_minutes: recipe_model.cook_time_minutes,
            author: recipe_model.author,
            author_id: recipe_model.author_id,
            is_private: recipe_model.is_private,
            translations: translation_dtos,
            tags,
            ingredient_groups,
            step_groups,
        })
    }
}

fn pick_translation(
    translations: &[RecipeTranslation],
    requested_lang: &str,
    fallback_lang: &str,
) -> Option<RecipeTranslation> {
    // First try requested language
    translations
        .iter()
        .find(|t| t.language_code == requested_lang)
        .cloned()
        // Then fallback
        .or_else(|| translations.iter().find(|t| t.language_code == fallback_lang).cloned())
}



impl Message for CreateRecipe {
    type Result = Result<RecipeResponse>;
}

impl Handler<CreateRecipe> for DbExecutor {
    type Result = Result<RecipeResponse>;

    fn handle(&mut self, msg: CreateRecipe, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;
        use crate::schema::recipe_translations;
        use std::fs;
        use std::path::PathBuf;
        use uuid::Uuid;

        // --- Ensure recipe image directory exists ---
        let image_dir = PathBuf::from("assets/recipes");
        fs::create_dir_all(&image_dir).map_err(|e| {
            log::error!("Failed to create image directory: {}", e);
            diesel::result::Error::RollbackTransaction
        })?;

        // --- Save main recipe image ---
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
        let url = format!("/assets/recipes/{}", file_name);

        // --- Get DB connection ---
        let mut conn = self.0.get()?;

        // --- Insert Recipe (without title/description) ---
        let new_recipe = NewRecipe {
            image_url: url.clone(),
            servings: msg.new_recipe.servings,
            prep_time_minutes: msg.new_recipe.prep_time_minutes,
            cook_time_minutes: msg.new_recipe.cook_time_minutes,
            author: msg.new_recipe.author.clone(),
            author_id: msg.new_recipe.author_id,
            is_private: msg.new_recipe.is_private,
        };

        let inserted_recipe: Recipe = diesel::insert_into(recipes)
            .values(&new_recipe)
            .get_result(&mut conn)?;

        // --- Insert Recipe Translations ---
        let mut translations_resp = Vec::new();
        for tr in msg.new_recipe.translations {
            diesel::insert_into(recipe_translations::table)
                .values((
                    recipe_translations::recipe_id.eq(inserted_recipe.id),
                    recipe_translations::language_code.eq(&tr.language_code),
                    recipe_translations::title.eq(&tr.title),
                    recipe_translations::description.eq(&tr.description),
                ))
                .execute(&mut conn)?;

            translations_resp.push(RecipeTranslationResponse {
                language_code: tr.language_code,
                title: tr.title,
                description: tr.description,
            });
        }

        // --- Tags ---
        let inserted_tags =
            create_or_associate_tags(&mut conn, inserted_recipe.id, msg.new_recipe.tags)?;

        // --- Ingredient groups (with translations) ---
        let inserted_ingredient_groups = create_ingredient_groups(
            &mut conn,
            inserted_recipe.id,
            msg.new_recipe.ingredient_groups,
            &*msg.language_code
        )?;

        // --- Step groups (with translations and images) ---
        let inserted_step_groups = create_step_groups(
            &mut conn,
            inserted_recipe.id,
            msg.new_recipe.step_groups,
            msg.step_images,
            msg.step_images_meta,
        )?;

        // --- Fetch with language_code to return response ---
        let ingredient_groups = fetch_ingredient_groups_for_recipe(
            &mut conn,
            inserted_recipe.id,
            Some(&msg.language_code),
            &*inserted_recipe.original_language_code
        )?;

        let step_groups = fetch_step_groups_for_recipe(
            &mut conn,
            inserted_recipe.id,
            Some(&msg.language_code),
            &*inserted_recipe.original_language_code
        )?;

        Ok(RecipeResponse::from_parts(
            inserted_recipe,
            translations_resp,                  // translations: if you want to fetch them later
            inserted_tags,           // tags
            ingredient_groups,       // ingredient groups
            step_groups,             // step groups
        ))
    }
}


impl Message for UpdateRecipe {
    type Result = Result<RecipeResponse>;
}

impl Handler<UpdateRecipe> for DbExecutor {
    type Result = Result<RecipeResponse>;

    fn handle(&mut self, msg: UpdateRecipe, _: &mut Self::Context) -> Self::Result {
        use crate::schema::{recipes, recipe_translations, recipe_versions};

        let mut conn = self.0.get()?;

        // --- Upload new main image if provided ---
        let new_image_url: String = if let Some(temp_file) = msg.main_image {
            upload_recipe_image(temp_file)?
        } else {
            recipes::table
                .find(msg.update_recipe.id)
                .select(recipes::image_url)
                .first::<String>(&mut conn)?
        };

        // --- Fetch current recipe state to version it ---
        let current_recipe: Recipe = recipes::table
            .find(msg.update_recipe.id)
            .first(&mut conn)?;

        // --- Save version before updating ---
        let version_data = serde_json::to_value(&current_recipe)
            .expect("Failed to serialize recipe for versioning");

        diesel::insert_into(recipe_versions::table)
            .values((
                recipe_versions::recipe_id.eq(current_recipe.id),
                recipe_versions::data.eq(version_data),
                recipe_versions::created_at.eq(chrono::Utc::now().naive_utc()),
                recipe_versions::edited_by.eq(msg.update_recipe.author_id.unwrap_or_default()),
            ))
            .execute(&mut conn)?;

        // --- Prepare changes for the main recipe ---
        let change = RecipeChange {
            servings: msg.update_recipe.servings,
            prep_time_minutes: msg.update_recipe.prep_time_minutes,
            cook_time_minutes: msg.update_recipe.cook_time_minutes,
            author: msg.update_recipe.author.clone(),
            author_id: msg.update_recipe.author_id,
            is_private: msg.update_recipe.is_private,
            image_url: new_image_url,
        };

        // --- Update the recipe itself ---
        let recipe: Recipe = diesel::update(recipes::table.find(msg.update_recipe.id))
            .set(change)
            .get_result(&mut conn)?;

        // --- Handle tags ---
        let tags = create_or_associate_tags(&mut conn, recipe.id, msg.update_recipe.tags.clone())?;

        // --- Sync ingredient groups and step groups ---
        let ingredient_groups = sync_ingredient_groups(
            &mut conn,
            recipe.id,
            msg.update_recipe.ingredient_groups.clone(),
            &*msg.language_code
        )?;

        let step_groups = sync_step_groups(
            &mut conn,
            recipe.id,
            msg.update_recipe.step_groups.clone(),
            msg.step_images,
            msg.step_images_meta.clone(),
            &*msg.language_code
        )?;

        // --- Fetch recipe translations ---
        let translations: Vec<RecipeTranslationResponse> = recipe_translations::table
            .filter(recipe_translations::recipe_id.eq(recipe.id))
            .load::<crate::models::RecipeTranslation>(&mut conn)?
            .into_iter()
            .map(|t| crate::dto::RecipeTranslationResponse {
                language_code: t.language_code,
                title: t.title,
                description: t.description,
            })
            .collect();

        Ok(RecipeResponse::from_parts(
            recipe,
            translations,
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
        use crate::schema::recipe_translations::dsl::*;


        let mut conn = self.0.get()?;
        let mut query = recipes.into_boxed();

        if !msg.include_private {
            query = query.filter(is_private.eq(false));
        }

        // 🔍 Apply filters
        if let Some(search) = &msg.filters.search {
            let pattern = format!("%{}%", search);

            // get recipe IDs that match the search in the specified language
            let matching_ids: Vec<Uuid> = recipe_translations
                .filter(language_code.eq(&msg.language_code))
                .filter(
                    title.ilike(&pattern)
                        .or(description.ilike(&pattern))
                )
                .select(recipe_id)
                .load(&mut conn)?;

            // filter recipes by those IDs
            query = query.filter(crate::schema::recipes::dsl::id.eq_any(matching_ids));
        }

        if let Some(min) = msg.filters.min_prep { query = query.filter(prep_time_minutes.ge(min)); }
        if let Some(max) = msg.filters.max_prep { query = query.filter(prep_time_minutes.le(max)); }
        if let Some(min) = msg.filters.min_cook { query = query.filter(cook_time_minutes.ge(min)); }
        if let Some(max) = msg.filters.max_cook { query = query.filter(cook_time_minutes.le(max)); }
        if let Some(from) = msg.filters.date_from { query = query.filter(crate::schema::recipes::dsl::created_at.ge(from.and_hms_opt(0, 0, 0).unwrap())); }
        if let Some(to) = msg.filters.date_to { query = query.filter(crate::schema::recipes::dsl::created_at.le(to.and_hms_opt(23, 59, 59).unwrap())); }

        let recipe_models: Vec<Recipe> = query.order(crate::schema::recipes::dsl::created_at.desc()).load(&mut conn)?;
        let mut result = Vec::with_capacity(recipe_models.len());

        for recipe in recipe_models {
            let other_recipe_id = recipe.id;

            // --- Fetch tags ---
            let tags = fetch_tags_for_recipe(&mut conn, other_recipe_id)?;

            // --- Fetch ingredient and step groups in the requested language ---
            let ingredient_groups = fetch_ingredient_groups_for_recipe(
                &mut conn,
                other_recipe_id,
                Some(&msg.language_code),
                &*recipe.original_language_code
            )?;
            let step_groups = fetch_step_groups_for_recipe(
                &mut conn,
                other_recipe_id,
                Some(&msg.language_code),
                &*recipe.original_language_code
            )?;

            // --- Fetch the translation for this language ---
            let translation = load_translation(&mut conn, &recipe, &msg.language_code)?;

            let translations = vec![RecipeTranslationResponse {
                language_code: translation.language_code,
                title: translation.title,
                description: translation.description,
            }];

            result.push(RecipeResponse::from_parts(
                recipe,
                translations,
                tags,
                ingredient_groups,
                step_groups,
            ));
        }

        Ok(result)
    }
}

fn load_translation(
    conn: &mut PgConnection,
    recipe: &Recipe,
    requested_lang: &str,
) -> QueryResult<RecipeTranslation> {
    use crate::schema::recipe_translations::dsl::*;

    // Try requested language first
    if let Ok(t) = recipe_translations
        .filter(recipe_id.eq(recipe.id))
        .filter(language_code.eq(requested_lang))
        .first::<RecipeTranslation>(conn)
    {
        return Ok(t);
    }

    // Fallback to original language
    recipe_translations
        .filter(recipe_id.eq(recipe.id))
        .filter(language_code.eq(&recipe.original_language_code))
        .first::<RecipeTranslation>(conn)
}




impl Message for GetAllRecipesByPage {
    type Result = Result<PaginatedRecipes>;
}

impl Handler<GetAllRecipesByPage> for DbExecutor {
    type Result = Result<PaginatedRecipes>;

    fn handle(&mut self, msg: GetAllRecipesByPage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipes::dsl::*;
        use crate::schema::recipe_translations::dsl::*;

        let mut conn = self.0.get()?;

        let filters = msg.filters.as_ref();
        let include_private = msg.include_private;

        let mut build_query = || {
            use crate::schema::recipes::dsl::*;

            let mut q = recipes.into_boxed();

            if !include_private {
                q = q.filter(is_private.eq(false));
            }

            if let Some(filters) = filters {
                if let Some(search) = &filters.search {
                    let pattern = format!("%{}%", search);

                    let matching_ids: Vec<uuid::Uuid> = recipe_translations
                        .filter(language_code.eq(&msg.language_code))
                        .filter(title.ilike(&pattern).or(description.ilike(&pattern)))
                        .select(recipe_id)
                        .load(&mut conn)
                        .unwrap_or_default();

                    q = q.filter(id.eq_any(matching_ids));
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
        let mut conn = self.0.get()?;
        // Total count
        let total: i64 = build_query().count().get_result(&mut conn)?;

        // Pagination
        let page = msg.page.unwrap_or(1).max(1);
        let per_page = msg.per_page.unwrap_or(10).max(1);
        let offset = (page - 1) * per_page;

        // Fetch page
        let recipe_models: Vec<Recipe> = build_query()
            .order(crate::schema::recipes::dsl::created_at.desc())
            .limit(per_page)
            .offset(offset)
            .load(&mut conn)?;

        let mut result = Vec::with_capacity(recipe_models.len());


        for recipe in recipe_models {
            let other_recipe_id = recipe.id;
            let tags = fetch_tags_for_recipe(&mut conn, other_recipe_id)?;
            let ingredient_groups = Vec::new(); // fetch_ingredient_groups_for_recipe(&mut conn, recipe_id)?;
            let step_groups = Vec::new(); // fetch_step_groups_for_recipe(&mut conn, recipe_id)?;

            let translation = load_translation(&mut conn, &recipe, &msg.language_code)?;

            let translations = vec![RecipeTranslationResponse {
                language_code: translation.language_code,
                title: translation.title,
                description: translation.description,
            }];

            result.push(RecipeResponse::from_parts(
                recipe,
                translations,
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
        use crate::schema::{favorites, recipes, recipe_translations};

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
            let recipe_id_val = recipe.id;

            let tags = fetch_tags_for_recipe(&mut conn, recipe_id_val)?;
            let ingredient_groups =
                fetch_ingredient_groups_for_recipe(
                    &mut conn,
                    recipe_id_val,
                    Some(&msg.language_code),
                    &*recipe.original_language_code
                )?;
            let step_groups =
                fetch_step_groups_for_recipe(
                    &mut conn,
                    recipe_id_val,
                    Some(&msg.language_code),
                    &*recipe.original_language_code
                )?;

            // --- Fetch translation for requested language ---
            let translations: Vec<RecipeTranslationResponse> = recipe_translations::table
                .filter(recipe_translations::recipe_id.eq(recipe_id_val))
                .filter(recipe_translations::language_code.eq(&msg.language_code))
                .load::<crate::models::RecipeTranslation>(&mut conn)?
                .into_iter()
                .map(|t| crate::dto::RecipeTranslationResponse {
                    language_code: t.language_code,
                    title: t.title,
                    description: t.description,
                })
                .collect();

            responses.push(
                RecipeResponse::from_parts(
                    recipe,
                    translations,
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
    type Result = Result<RecipeRatingResponse, DbError>;
}

impl Handler<GetRecipeRating> for DbExecutor {
    type Result = Result<RecipeRatingResponse, DbError>;

    fn handle(&mut self, msg: GetRecipeRating, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipe_ratings::dsl::*;
        use diesel::dsl::sql;
        use diesel::sql_types::{Double, Nullable};

        let mut conn = self.0.get().map_err(DbError::from)?;

        let result = recipe_ratings
            .filter(recipe_id.eq(msg.recipe_id))
            .select((sql::<Nullable<Double>>("AVG(rating)::float8"), diesel::dsl::count_star()))
            .first::<(Option<f64>, i64)>(&mut conn)
            .optional()
            .map_err(DbError::from)?;

        let (avg, count) = result.unwrap_or((Some(0.0), 0));



        // User-specific rating
        let user_rating = if let Some(uid) = msg.user_id {
            recipe_ratings
                .filter(recipe_id.eq(msg.recipe_id))
                .filter(user_id.eq(uid))
                .select(rating)
                .first::<i32>(&mut conn)
                .optional()
                .map_err(DbError::from)?
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
    type Result = Result<RecipeCommentResponse, DbError>;
}

impl Handler<CreateComment> for DbExecutor {
    type Result = Result<RecipeCommentResponse, DbError>;

    fn handle(&mut self, msg: CreateComment, _: &mut Self::Context) -> Self::Result {
        use crate::schema::recipe_comments::dsl::*;
        use crate::schema::users::dsl as users_dsl;
        use crate::schema::users::dsl::username as username_dsl;

        let mut conn = self.0.get()?;

        // Insert comment
        let comment = diesel::insert_into(recipe_comments)
            .values(NewRecipeComment {
                recipe_id: msg.recipe_id,
                user_id: msg.user_id.unwrap(),
                parent_id: msg.parent_id,
                content: msg.content,
            })
            .get_result::<RecipeComment>(&mut conn)?;

        // Fetch username
        let other_username: String = users_dsl::users
            .filter(users_dsl::id.eq(comment.user_id))
            .select(username_dsl)
            .first(&mut conn)?;

        Ok(RecipeCommentResponse {
            id: comment.id,
            user_id: comment.user_id,
            username: other_username,
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
        use crate::schema::users::dsl as users_dsl;

        let mut conn = self.0.get()?;

        let rows: Vec<(RecipeComment, String)> = recipe_comments
            .inner_join(users::table.on(users_dsl::id.eq(user_id)))
            .filter(recipe_id.eq(msg.recipe_id))
            .filter(deleted_at.is_null())
            .order(created_at.asc())
            .select((recipe_comments::all_columns(), users_dsl::username))
            .load(&mut conn)?;

        // 1️⃣ Build all nodes
        let mut map: HashMap<Uuid, RecipeCommentResponse> = HashMap::new();
        for (c, uname) in &rows {
            map.insert(
                c.id,
                RecipeCommentResponse {
                    id: c.id,
                    user_id: c.user_id,
                    username: uname.clone(),
                    content: c.content.clone(),
                    created_at: c.created_at,
                    edited_at: c.edited_at,
                    children: Vec::new(),
                },
            );
        }

        // 2️⃣ Group children by parent_id
        let mut children_map: HashMap<Uuid, Vec<RecipeCommentResponse>> = HashMap::new();
        for (c, _) in &rows {
            if let Some(pid) = c.parent_id {
                if let Some(child) = map.get(&c.id).cloned() {
                    children_map.entry(pid).or_default().push(child);
                }
            }
        }

        // 3️⃣ Recursively attach children
        fn attach_children(
            node: &mut RecipeCommentResponse,
            children_map: &mut HashMap<Uuid, Vec<RecipeCommentResponse>>,
        ) {
            if let Some(mut kids) = children_map.remove(&node.id) {
                for kid in &mut kids {
                    attach_children(kid, children_map);
                }
                node.children = kids;
            }
        }

        // 4️⃣ Build root nodes
        let mut roots: Vec<RecipeCommentResponse> = Vec::new();
        for (c, _) in &rows {
            if c.parent_id.is_none() {
                if let Some(mut node) = map.get(&c.id).cloned() {
                    attach_children(&mut node, &mut children_map);
                    roots.push(node);
                }
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
        use crate::schema::recipe_translations::dsl as translations_dsl;
        use crate::schema::recipe_versions::dsl as versions_dsl;

        let mut conn = self.0.get()?;

        // 1️⃣ Fetch version JSON
        let row: (uuid::Uuid, uuid::Uuid, serde_json::Value, chrono::NaiveDateTime, uuid::Uuid) =
            versions_dsl::recipe_versions
                .filter(versions_dsl::id.eq(msg.version_id))
                .select((
                    versions_dsl::id,
                    versions_dsl::recipe_id,
                    versions_dsl::data,
                    versions_dsl::created_at,
                    versions_dsl::edited_by,
                ))
                .first(&mut conn)?;

        // Deserialize the core recipe fields
        let version_recipe: Recipe = serde_json::from_value(row.2)
            .expect("Failed to deserialize recipe JSON for restoration");

        // 2️⃣ Restore main recipe fields (excluding title/description)
        let restored_recipe: Recipe = diesel::update(recipes.find(row.1))
            .set((
                image_url.eq(version_recipe.image_url),
                servings.eq(version_recipe.servings),
                prep_time_minutes.eq(version_recipe.prep_time_minutes),
                cook_time_minutes.eq(version_recipe.cook_time_minutes),
                author.eq(version_recipe.author),
                author_id.eq(version_recipe.author_id),
                is_private.eq(version_recipe.is_private),
            ))
            .get_result(&mut conn)?;

        // 3️⃣ Fetch translations from the separate table
        let translations: Vec<RecipeTranslationResponse> = translations_dsl::recipe_translations
            .filter(translations_dsl::recipe_id.eq(row.1))
            .load::<crate::models::RecipeTranslation>(&mut conn)?
            .into_iter()
            .map(|t| crate::dto::RecipeTranslationResponse {
                language_code: t.language_code,
                title: t.title,
                description: t.description,
            })
            .collect();

        // 4️⃣ Fetch tags, ingredient groups, step groups
        let tags = fetch_tags_for_recipe(&mut conn, row.1)?;
        let ingredient_groups =
            fetch_ingredient_groups_for_recipe(
                &mut conn,
                row.1,
                Some(&msg.language_code),
                &*restored_recipe.original_language_code
            )?;
        let step_groups =
            fetch_step_groups_for_recipe(
                &mut conn,
                row.1,
                Some(&msg.language_code),
                &*restored_recipe.original_language_code
            )?;

        // 5️⃣ Return restored recipe response
        Ok(RecipeResponse::from_parts(
            restored_recipe,
            translations,
            tags,
            ingredient_groups,
            step_groups,
        ))
    }
}
