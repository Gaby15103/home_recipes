use crate::dto::comment_dto::{CommentDto, CreateCommentDto};
use crate::dto::ingredient_group_dto::IngredientGroupViewDto;
use crate::dto::recipe_dto::{CreateRecipeInput, EditRecipeInput, GetAllRecipesByPageQuery, RecipeFilter, RecipeFilterByPage, RecipeViewDto};
use crate::dto::recipe_rating_dto::RecipeRatingDto;
use crate::dto::step_group_dto::StepGroupViewDto;
use crate::dto::tag_dto::{InputTag, TagDto};
use crate::errors::Error;
use crate::repositories::{ingredient_group_repository, step_group_repository, tag_repository};
use chrono::Utc;
use entity::{favorites, ingredient_groups, ingredient_translations, ingredients, recipe_analytics, recipe_comments, recipe_ratings, recipe_tags, recipe_translations, recipes, step_groups, steps, users};
use futures_util::TryFutureExt;
use migration::JoinType;
use sea_orm::{ActiveModelTrait, ColumnTrait, DeleteResult, FromQueryResult, PaginatorTrait, SelectExt, Set, TransactionError, TransactionTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{ExprTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait};
use serde_json::json;
use std::collections::HashMap;
use std::ops::Deref;
use sea_orm::sea_query::Expr;
use uuid::Uuid;

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<recipes::Model>, Error> {
    recipes::Entity::find()
        .all(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch all recipes",
            "operation": "find_all",
            "entity": "recipes",
            "error": e.to_string(),
            "stage": "database_query"
        })))
}

pub async fn find_by_query(
    db: &DatabaseConnection,
    filter: RecipeFilter,
    lang_code: &str,
) -> Result<Option<Vec<recipes::Model>>, Error> {
    let mut query = recipes::Entity::find();

    if !filter.scope {
        query = query.filter(recipes::Column::IsPrivate.eq(false));
    }

    if let Some(s) = &filter.search {
        if !s.trim().is_empty() {
            let pattern = format!("%{}%", s);
            query = query
                .join(
                    JoinType::LeftJoin,
                    recipes::Relation::RecipeTranslations.def(),
                )
                .filter(
                    recipe_translations::Column::LanguageCode.eq(lang_code).and(
                        recipe_translations::Column::Title
                            .like(&pattern)
                            .or(recipe_translations::Column::Description.like(&pattern)),
                    ),
                );
        }
    }

    if let Some(ingredients_list) = &filter.ingredient {
        if !ingredients_list.is_empty() {
            query = query
                .join(
                    JoinType::InnerJoin,
                    recipes::Relation::IngredientGroups.def(),
                )
                .join(
                    JoinType::InnerJoin,
                    ingredient_groups::Relation::Ingredients.def(),
                )
                .join(
                    JoinType::InnerJoin,
                    ingredient_translations::Relation::Ingredients.def(),
                )
                .filter(ingredient_translations::Column::LanguageCode.eq(lang_code));

            let mut condition = sea_orm::Condition::any();
            for ing in ingredients_list {
                if !ing.trim().is_empty() {
                    condition = condition
                        .add(ingredient_translations::Column::Data.like(format!("%{}%", ing)));
                }
            }
            query = query.filter(condition);
        }
    }

    query = query
        .group_by(recipes::Column::Id)
        .order_by_desc(recipes::Column::CreatedAt);

    let results = query
        .all(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to execute recipe search query",
            "operation": "find_by_query",
            "entity": "recipes",
            "language_code": lang_code,
            "search_term": filter.search.as_deref().unwrap_or(""),
            "has_ingredients_filter": filter.ingredient.is_some(),
            "scope": filter.scope,
            "error": e.to_string(),
            "stage": "complex_query"
        })))?;

    if results.is_empty() {
        Ok(None)
    } else {
        Ok(Some(results))
    }
}

pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<recipes::Model, Error> {
    recipes::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch recipe by ID",
            "operation": "find_by_id",
            "entity": "recipes",
            "id": id.to_string(),
            "error": e.to_string(),
            "stage": "database_query"
        })))?
        .ok_or_else(|| Error::InternalServerError(json!({
            "message": "Recipe not found in database",
            "operation": "find_by_id",
            "entity": "recipes",
            "id": id.to_string(),
            "error": "Record does not exist",
            "stage": "validation"
        })))
}
#[derive(FromQueryResult)]
struct Counts {
    nb_ingredients: i64, // Use i64 because count() usually returns i64
    nb_steps: i64,
}

pub async fn get_recipe_counts(
    db: &DatabaseConnection,
    recipe_id: Uuid
) -> Result<Option<(i32, i32)>, Error> {
    let step_count = ingredient_groups::Entity::find()
        .filter(ingredient_groups::Column::RecipeId.eq(recipe_id))
        .join(JoinType::InnerJoin, ingredient_groups::Relation::Ingredients.def())
        .count(db)
        .await?;

    let ingredient_count = step_groups::Entity::find()
        .filter(step_groups::Column::RecipeId.eq(recipe_id))
        .join(JoinType::InnerJoin, step_groups::Relation::Steps.def())
        .count(db)
        .await?;

    let counts = Some((step_count as i32, ingredient_count as i32));
    Ok(counts)
}

pub async fn find_by_author(
    db: &DatabaseConnection,
    author_id: Uuid
) -> Result<Vec<recipes::Model>, Error> {
    recipes::Entity::find()
        .filter(recipes::Column::AuthorId.eq(author_id))
        .all(db)
        .await
        .map_err(Error::from)
}

pub async fn create(
    db: &DatabaseConnection,
    new_recipe: CreateRecipeInput,
    preferred_language: &str,
) -> Result<RecipeViewDto, TransactionError<Error>> {
    let pref_lang = preferred_language.to_string();
    db.transaction::<_, RecipeViewDto, Error>(|txn| {
        Box::pin(async move {
            let recipe_model = recipes::ActiveModel {
                image_url: Set(new_recipe.image_url.clone()),
                author_id: Set(new_recipe.author_id),
                author: Set(new_recipe.author.clone()),
                servings: Set(new_recipe.servings),
                prep_time_minutes: Set(new_recipe.prep_time_minutes),
                cook_time_minutes: Set(new_recipe.cook_time_minutes),
                is_private: Set(new_recipe.is_private),
                original_language_code: Set(new_recipe.primary_language.clone()),
                ..Default::default()
            }
                .insert(txn)
                .await
                .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to insert recipe into database",
                "operation": "create",
                "entity": "recipes",
                "author_id": new_recipe.author_id.expect("REASON").to_string(),
                "author": &new_recipe.author,
                "servings": new_recipe.servings,
                "error": e.to_string(),
                "stage": "recipe_insert"
            })))?;

            for (idx, trans) in new_recipe.translations.iter().enumerate() {
                recipe_translations::ActiveModel {
                    recipe_id: Set(recipe_model.id),
                    language_code: Set(trans.language_code.clone()),
                    title: Set(trans.title.clone()),
                    description: Set(trans.description.clone()),
                    ..Default::default()
                }
                    .insert(txn)
                    .await
                    .map_err(|e| Error::InternalServerError(json!({
                    "message": "Failed to insert recipe translation",
                    "operation": "create",
                    "entity": "recipe_translations",
                    "recipe_id": recipe_model.id.to_string(),
                    "language_code": &trans.language_code,
                    "translation_index": idx,
                    "error": e.to_string(),
                    "stage": "translation_insert"
                })))?;
            }

            let inserted_tags: Vec<TagDto> =
                tag_repository::find_or_create_tags(txn, new_recipe.tags.clone(), recipe_model.id)
                    .await
                    .map_err(|e| {
                        log::error!("Failed to create tags for recipe {}: {:?}", recipe_model.id, e);
                        match e {
                            Error::InternalServerError(mut ctx) => {
                                ctx["recipe_id"] = json!(recipe_model.id.to_string());
                                Error::InternalServerError(ctx)
                            }
                            other => other,
                        }
                    })?;

            let inserted_ingredient_group: Vec<IngredientGroupViewDto> =
                ingredient_group_repository::create_multiple(
                    &txn,
                    recipe_model.id,
                    new_recipe.ingredient_groups.clone(),
                    recipe_model.original_language_code.deref(),
                )
                    .await
                    .map_err(|e| {
                        log::error!("Failed to create ingredient groups for recipe {}: {:?}", recipe_model.id, e);
                        match e {
                            Error::InternalServerError(mut ctx) => {
                                ctx["recipe_id"] = json!(recipe_model.id.to_string());
                                ctx["stage"] = json!("ingredient_group_creation");
                                Error::InternalServerError(ctx)
                            }
                            other => other,
                        }
                    })?;

            let inserted_step_group: Vec<StepGroupViewDto> =
                step_group_repository::create_multiple(
                    &txn,
                    recipe_model.id,
                    new_recipe.step_groups.clone(),
                    recipe_model.original_language_code.deref(),
                )
                    .await
                    .map_err(|e| {
                        log::error!("Failed to create step groups for recipe {}: {:?}", recipe_model.id, e);
                        match e {
                            Error::InternalServerError(mut ctx) => {
                                ctx["recipe_id"] = json!(recipe_model.id.to_string());
                                ctx["stage"] = json!("step_group_creation");
                                Error::InternalServerError(ctx)
                            }
                            other => other,
                        }
                    })?;

            let translations = recipe_translations::Entity::find()
                .filter(recipe_translations::Column::RecipeId.eq(recipe_model.id))
                .all(txn)
                .await
                .map_err(|e| Error::InternalServerError(json!({
                    "message": "Failed to fetch recipe translations after creation",
                    "operation": "create",
                    "entity": "recipe_translations",
                    "recipe_id": recipe_model.id.to_string(),
                    "error": e.to_string(),
                    "stage": "fetch_translations"
                })))?;

            let main_trans = translations
                .iter()
                .find(|t| t.language_code == pref_lang)
                .or_else(|| {
                    translations
                        .iter()
                        .find(|t| t.language_code == recipe_model.original_language_code)
                })
                .cloned()
                .ok_or_else(|| Error::InternalServerError(json!({
                    "message": "No suitable translation found for recipe",
                    "operation": "create",
                    "recipe_id": recipe_model.id.to_string(),
                    "preferred_language": &pref_lang,
                    "original_language": &recipe_model.original_language_code,
                    "available_translations": translations.iter().map(|t| &t.language_code).collect::<Vec<_>>(),
                    "stage": "translation_selection"
                })))?;

            Ok(RecipeViewDto::build(
                recipe_model,
                main_trans,
                inserted_tags,
                inserted_ingredient_group,
                inserted_step_group,
            ))
        })
    })
        .await
        .map_err(|e| {
            log::error!("Transaction failed in create recipe: {:?}", e);
            e
        })
}

pub async fn find_latest_public(
    db: &DatabaseConnection,
    limit: i64,
) -> Result<Vec<recipes::Model>, Error> {
    recipes::Entity::find()
        .filter(recipes::Column::IsPrivate.eq(false))
        .order_by_desc(recipes::Column::CreatedAt)
        .limit(limit as u64)
        .all(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch latest public recipes",
            "operation": "find_latest_public",
            "entity": "recipes",
            "limit": limit,
            "error": e.to_string(),
            "stage": "database_query"
        })))
}
pub async fn find_latest_work(
    db: &DatabaseConnection,
    limit: i64,
    user_id: Uuid,
) -> Result<Vec<recipes::Model>, Error> {
    recipes::Entity::find()
        .filter(recipes::Column::AuthorId.eq(user_id))
        .order_by_desc(recipes::Column::CreatedAt)
        .limit(limit as u64)
        .all(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch latest works",
            "operation": "find_latest_works",
            "entity": "recipes",
            "limit": limit,
            "error": e.to_string(),
            "stage": "database_query"
        })))
}

pub async fn get_dashboard_stats(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<(i64, i64, i64), Error> {

    let total = recipes::Entity::find()
        .filter(recipes::Column::AuthorId.eq(user_id))
        .count(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to count total recipes",
            "error": e.to_string()
        })))?;

    let public = recipes::Entity::find()
        .filter(recipes::Column::AuthorId.eq(user_id))
        .filter(recipes::Column::IsPrivate.eq(false))
        .count(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to count public recipes",
            "error": e.to_string()
        })))?;


    let total_views = recipes::Entity::find()
        .select_only()
        .column_as(recipe_analytics::Column::Id.count(), "total_views")
        .inner_join(recipe_analytics::Entity)
        .filter(recipes::Column::AuthorId.eq(user_id))
        .into_tuple::<i64>()
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to aggregate recipe views",
            "operation": "get_dashboard_stats",
            "user_id": user_id.to_string(),
            "error": e.to_string()
        })))?
        .unwrap_or(0);

    Ok((total as i64, public as i64, total_views))
}
pub async fn get_by_author_and_filter(
    db: &DatabaseConnection,
    user_id: Uuid,
    query_params: GetAllRecipesByPageQuery,
    lang_code: &String,
) -> Result<Vec<recipes::Model>, Error> {
    let mut query = recipes::Entity::find()
        .filter(recipes::Column::AuthorId.eq(user_id));

    let page = query_params.page.unwrap_or(1) as u64;
    let per_page = query_params.per_page.unwrap_or(10) as u64;

    if let Some(filter) = query_params.filters {

        if let Some(s) = &filter.search {
            if !s.trim().is_empty() {
                let pattern = format!("%{}%", s);
                query = query
                    .join(JoinType::LeftJoin, recipes::Relation::RecipeTranslations.def())
                    .filter(
                        recipe_translations::Column::LanguageCode.eq(lang_code).and(
                            recipe_translations::Column::Title.like(&pattern)
                                .or(recipe_translations::Column::Description.like(&pattern)),
                        ),
                    );
            }
        }

        if let Some(min) = filter.min_prep { query = query.filter(recipes::Column::PrepTimeMinutes.gte(min)); }
        if let Some(max) = filter.max_prep { query = query.filter(recipes::Column::PrepTimeMinutes.lte(max)); }

        if let Some(min) = filter.min_cook { query = query.filter(recipes::Column::CookTimeMinutes.gte(min)); }
        if let Some(max) = filter.max_cook { query = query.filter(recipes::Column::CookTimeMinutes.lte(max)); }

        if let Some(from) = filter.date_from { query = query.filter(recipes::Column::UpdatedAt.gte(from)); }
        if let Some(to) = filter.date_to { query = query.filter(recipes::Column::UpdatedAt.lte(to)); }

        if let Some(ingredients_list) = &filter.ingredient {
            if !ingredients_list.is_empty() {
                query = query
                    .join(JoinType::InnerJoin, recipes::Relation::IngredientGroups.def())
                    .join(JoinType::InnerJoin, ingredient_groups::Relation::Ingredients.def())
                    .join(JoinType::InnerJoin, ingredient_translations::Relation::Ingredients.def())
                    .filter(ingredient_translations::Column::LanguageCode.eq(lang_code));

                let mut condition = sea_orm::Condition::any();
                for ing in ingredients_list {
                    if !ing.trim().is_empty() {
                        condition = condition.add(ingredient_translations::Column::Data.like(format!("%{}%", ing)));
                    }
                }
                query = query.filter(condition);
            }
        }
    }

    let paginator = query
        .group_by(recipes::Column::Id)
        .order_by_desc(recipes::Column::CreatedAt)
        .paginate(db, per_page);

    let results = paginator
        .fetch_page(page - 1)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch filtered recipes for author",
            "operation": "get_by_author_and_filter",
            "user_id": user_id.to_string(),
            "page": page,
            "error": e.to_string(),
            "stage": "database_query"
        })))?;

    Ok(results)
}
pub async fn find_by_query_by_page(
    db: &DatabaseConnection,
    filter: RecipeFilterByPage,
    lang_code: &str,
) -> Result<Option<Vec<recipes::Model>>, Error> {
    // Extract pagination info BEFORE filter is moved
    let page = filter.page.unwrap_or(1);
    let per_page = filter.per_page.unwrap_or(10);
    let has_filters = filter.filters.is_some();

    let mut query = recipes::Entity::find();

    if let Some(filter) = filter.filters {
        query = query.filter(recipes::Column::IsPrivate.eq(!filter.scope));

        if let Some(s) = &filter.search {
            if !s.trim().is_empty() {
                let pattern = format!("%{}%", s);
                query = query
                    .join(
                        JoinType::LeftJoin,
                        recipes::Relation::RecipeTranslations.def(),
                    )
                    .filter(
                        recipe_translations::Column::LanguageCode.eq(lang_code).and(
                            recipe_translations::Column::Title
                                .like(&pattern)
                                .or(recipe_translations::Column::Description.like(&pattern)),
                        ),
                    );
            }
        }

        if let Some(ingredients_list) = &filter.ingredient {
            if !ingredients_list.is_empty() {
                query = query
                    .join(
                        JoinType::InnerJoin,
                        recipes::Relation::IngredientGroups.def(),
                    )
                    .join(
                        JoinType::InnerJoin,
                        ingredient_groups::Relation::Ingredients.def(),
                    )
                    .join(
                        JoinType::InnerJoin,
                        ingredient_translations::Relation::Ingredients.def(),
                    )
                    .filter(ingredient_translations::Column::LanguageCode.eq(lang_code));

                let mut condition = sea_orm::Condition::any();
                for ing in ingredients_list {
                    if !ing.trim().is_empty() {
                        condition = condition
                            .add(ingredient_translations::Column::Data.like(format!("%{}%", ing)));
                    }
                }
                query = query.filter(condition);
            }
        }
    }

    query = query
        .group_by(recipes::Column::Id)
        .order_by_desc(recipes::Column::CreatedAt);

    let paginator = query.paginate(db, per_page as u64);
    let results = paginator
        .fetch_page((page - 1) as u64)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch paginated recipes",
            "operation": "find_by_query_by_page",
            "entity": "recipes",
            "page": page,
            "per_page": per_page,
            "language_code": lang_code,
            "has_filters": has_filters,
            "error": e.to_string(),
            "stage": "pagination_query"
        })))?;

    if results.is_empty() {
        Ok(None)
    } else {
        Ok(Some(results))
    }
}

pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, Error> {
    recipes::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to delete recipe",
            "operation": "delete",
            "entity": "recipes",
            "id": id.to_string(),
            "error": e.to_string(),
            "stage": "delete"
        })))
}

pub async fn get_favorites(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<recipes::Model>, Error> {
    recipes::Entity::find()
        .join(JoinType::InnerJoin, recipes::Relation::Favorites.def())
        .filter(favorites::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch user favorites",
            "operation": "get_favorites",
            "entity": "recipes",
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "favorites_query"
        })))
}

pub async fn update(
    db: &DatabaseConnection,
    updated_recipe: EditRecipeInput,
    recipe_id: Uuid,
    lang_code: &str,
) -> Result<(), TransactionError<Error>> {
    let lang_code_owned = lang_code.to_string();

    db.transaction::<_, (), Error>(|txn| {
        let lang_code = lang_code_owned.clone();

        Box::pin(async move {
            let original_recipe = recipes::Entity::find_by_id(recipe_id)
                .one(txn)
                .await
                .map_err(|e| Error::InternalServerError(json!({
                    "message": "Failed to fetch recipe for update",
                    "operation": "update",
                    "entity": "recipes",
                    "recipe_id": recipe_id.to_string(),
                    "error": e.to_string(),
                    "stage": "fetch_original"
                })))?
                .ok_or_else(|| Error::InternalServerError(json!({
                    "message": "Recipe not found for update",
                    "operation": "update",
                    "entity": "recipes",
                    "recipe_id": recipe_id.to_string(),
                    "stage": "validation"
                })))?;

            let mut active_model: recipes::ActiveModel = original_recipe.clone().into();
            let mut base_changed = false;

            if original_recipe.image_url != updated_recipe.image_url {
                active_model.image_url = Set(updated_recipe.image_url.clone());
                base_changed = true;
            }
            if original_recipe.servings != updated_recipe.servings {
                active_model.servings = Set(updated_recipe.servings);
                base_changed = true;
            }
            if original_recipe.prep_time_minutes != updated_recipe.prep_time_minutes {
                active_model.prep_time_minutes = Set(updated_recipe.prep_time_minutes);
                base_changed = true;
            }
            if original_recipe.cook_time_minutes != updated_recipe.cook_time_minutes {
                active_model.cook_time_minutes = Set(updated_recipe.cook_time_minutes);
                base_changed = true;
            }
            if original_recipe.is_private != updated_recipe.is_private {
                active_model.is_private = Set(updated_recipe.is_private);
                base_changed = true;
            }

            let recipe_model = if base_changed {
                active_model
                    .update(txn)
                    .await
                    .map_err(|e| Error::InternalServerError(json!({
                        "message": "Failed to update recipe base fields",
                        "operation": "update",
                        "entity": "recipes",
                        "recipe_id": recipe_id.to_string(),
                        "error": e.to_string(),
                        "stage": "base_update"
                    })))?
            } else {
                original_recipe
            };

            let incoming_ids: Vec<Uuid> = updated_recipe
                .translations
                .iter()
                .filter_map(|t| t.id)
                .collect();

            recipe_translations::Entity::delete_many()
                .filter(recipe_translations::Column::RecipeId.eq(recipe_id))
                .filter(recipe_translations::Column::Id.is_not_in(incoming_ids))
                .exec(txn)
                .await
                .map_err(|e| Error::InternalServerError(json!({
                    "message": "Failed to delete removed translations",
                    "operation": "update",
                    "entity": "recipe_translations",
                    "recipe_id": recipe_id.to_string(),
                    "error": e.to_string(),
                    "stage": "delete_old_translations"
                })))?;

            for (trans_idx, trans_input) in updated_recipe.translations.iter().enumerate() {
                match trans_input.id {
                    Some(existing_id) => {
                        let existing_trans = recipe_translations::Entity::find_by_id(existing_id)
                            .one(txn)
                            .await
                            .map_err(|e| Error::InternalServerError(json!({
                                "message": "Failed to fetch translation for update",
                                "operation": "update",
                                "entity": "recipe_translations",
                                "translation_id": existing_id.to_string(),
                                "recipe_id": recipe_id.to_string(),
                                "error": e.to_string(),
                                "stage": "fetch_translation"
                            })))?
                            .ok_or_else(|| Error::InternalServerError(json!({
                                "message": "Translation not found for update",
                                "operation": "update",
                                "entity": "recipe_translations",
                                "translation_id": existing_id.to_string(),
                                "recipe_id": recipe_id.to_string(),
                                "stage": "validation"
                            })))?;

                        if existing_trans.title != trans_input.title
                            || existing_trans.description != trans_input.description
                            || existing_trans.language_code != trans_input.language_code
                        {
                            let mut trans_active: recipe_translations::ActiveModel =
                                existing_trans.into();
                            trans_active.language_code = Set(trans_input.language_code.clone());
                            trans_active.title = Set(trans_input.title.clone());
                            trans_active.description = Set(trans_input.description.clone());
                            trans_active
                                .update(txn)
                                .await
                                .map_err(|e| Error::InternalServerError(json!({
                                    "message": "Failed to update translation",
                                    "operation": "update",
                                    "entity": "recipe_translations",
                                    "translation_id": existing_id.to_string(),
                                    "recipe_id": recipe_id.to_string(),
                                    "translation_index": trans_idx,
                                    "error": e.to_string(),
                                    "stage": "translation_update"
                                })))?;
                        }
                    }
                    None => {
                        recipe_translations::ActiveModel {
                            id: Set(Uuid::new_v4()),
                            recipe_id: Set(recipe_id),
                            language_code: Set(trans_input.language_code.clone()),
                            title: Set(trans_input.title.clone()),
                            description: Set(trans_input.description.clone()),
                            ..Default::default()
                        }
                            .insert(txn)
                            .await
                            .map_err(|e| Error::InternalServerError(json!({
                            "message": "Failed to insert new translation",
                            "operation": "update",
                            "entity": "recipe_translations",
                            "recipe_id": recipe_id.to_string(),
                            "language_code": &trans_input.language_code,
                            "translation_index": trans_idx,
                            "error": e.to_string(),
                            "stage": "new_translation_insert"
                        })))?;
                    }
                }
            }

            let incoming_existing_tag_ids: Vec<Uuid> = updated_recipe
                .tags
                .iter()
                .filter_map(|t| {
                    if let InputTag::Existing { id } = t {
                        Some(*id)
                    } else {
                        None
                    }
                })
                .collect();

            recipe_tags::Entity::delete_many()
                .filter(recipe_tags::Column::RecipeId.eq(recipe_id))
                .filter(recipe_tags::Column::TagId.is_not_in(incoming_existing_tag_ids))
                .exec(txn)
                .await
                .map_err(|e| Error::InternalServerError(json!({
                    "message": "Failed to delete removed tags",
                    "operation": "update",
                    "entity": "recipe_tags",
                    "recipe_id": recipe_id.to_string(),
                    "error": e.to_string(),
                    "stage": "delete_old_tags"
                })))?;

            tag_repository::find_or_create_tags(txn, updated_recipe.tags.clone(), recipe_id)
                .await
                .map_err(|e| {
                    log::error!("Failed to update tags for recipe {}: {:?}", recipe_id, e);
                    match e {
                        Error::InternalServerError(mut ctx) => {
                            ctx["recipe_id"] = json!(recipe_id.to_string());
                            ctx["stage"] = json!("tag_update");
                            Error::InternalServerError(ctx)
                        }
                        other => other,
                    }
                })?;

            let incoming_group_ids: Vec<Uuid> = updated_recipe
                .ingredient_groups
                .iter()
                .filter_map(|g| g.id)
                .collect();

            ingredient_groups::Entity::delete_many()
                .filter(ingredient_groups::Column::RecipeId.eq(recipe_id))
                .filter(ingredient_groups::Column::Id.is_not_in(incoming_group_ids))
                .exec(txn)
                .await
                .map_err(|e| Error::InternalServerError(json!({
                    "message": "Failed to delete removed ingredient groups",
                    "operation": "update",
                    "entity": "ingredient_groups",
                    "recipe_id": recipe_id.to_string(),
                    "error": e.to_string(),
                    "stage": "delete_old_ingredient_groups"
                })))?;

            ingredient_group_repository::update(txn, recipe_id, updated_recipe.ingredient_groups.clone())
                .await
                .map_err(|e| {
                    log::error!("Failed to update ingredient groups for recipe {}: {:?}", recipe_id, e);
                    match e {
                        Error::InternalServerError(mut ctx) => {
                            ctx["recipe_id"] = json!(recipe_id.to_string());
                            ctx["stage"] = json!("ingredient_group_update");
                            Error::InternalServerError(ctx)
                        }
                        other => other,
                    }
                })?;

            step_group_repository::update(txn, recipe_id, updated_recipe.step_groups.clone())
                .await
                .map_err(|e| {
                    log::error!("Failed to update step groups for recipe {}: {:?}", recipe_id, e);
                    match e {
                        Error::InternalServerError(mut ctx) => {
                            ctx["recipe_id"] = json!(recipe_id.to_string());
                            ctx["stage"] = json!("step_group_update");
                            Error::InternalServerError(ctx)
                        }
                        other => other,
                    }
                })?;

            Ok(())
        })
    })
        .await
        .map_err(|e| {
            log::error!("Transaction failed in update recipe {}: {:?}", recipe_id, e);
            e
        })
}

pub async fn get_analytics(db: &DatabaseConnection, recipe_id: Uuid) -> Result<u64, Error> {
    recipe_analytics::Entity::find()
        .filter(recipe_analytics::Column::RecipeId.eq(recipe_id))
        .count(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch recipe analytics",
            "operation": "get_analytics",
            "entity": "recipe_analytics",
            "recipe_id": recipe_id.to_string(),
            "error": e.to_string(),
            "stage": "count_query"
        })))
}

pub async fn add_view(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<(), Error> {
    recipe_analytics::ActiveModel {
        recipe_id: Set(recipe_id),
        user_id: Set(user_id),
        ..Default::default()
    }
        .insert(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to record recipe view",
        "operation": "add_view",
        "entity": "recipe_analytics",
        "recipe_id": recipe_id.to_string(),
        "user_id": user_id.map(|u| u.to_string()),
        "error": e.to_string(),
        "stage": "insert"
    })))?;

    Ok(())
}

pub async fn toogle_favorite(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Uuid,
) -> Result<bool, Error> {
    let favorited = favorites::Entity::find()
        .filter(favorites::Column::RecipeId.eq(recipe_id))
        .filter(favorites::Column::UserId.eq(user_id))
        .exists(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to check favorite status",
            "operation": "toggle_favorite",
            "entity": "favorites",
            "recipe_id": recipe_id.to_string(),
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "check_exists"
        })))?;

    if favorited {
        let res = favorites::Entity::delete_many()
            .filter(favorites::Column::RecipeId.eq(recipe_id))
            .filter(favorites::Column::UserId.eq(user_id))
            .exec(db)
            .await
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to remove favorite",
                "operation": "toggle_favorite",
                "entity": "favorites",
                "recipe_id": recipe_id.to_string(),
                "user_id": user_id.to_string(),
                "error": e.to_string(),
                "stage": "delete"
            })))?;

        Ok(!(res.rows_affected > 0))
    } else {
        favorites::ActiveModel {
            user_id: Set(user_id),
            recipe_id: Set(recipe_id),
            ..Default::default()
        }
            .insert(db)
            .await
            .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to add favorite",
            "operation": "toggle_favorite",
            "entity": "favorites",
            "recipe_id": recipe_id.to_string(),
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "insert"
        })))?;

        Ok(true)
    }
}

pub async fn rate(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Uuid,
    rating: i32,
) -> Result<(), Error> {
    let existing_rating = recipe_ratings::Entity::find()
        .filter(recipe_ratings::Column::RecipeId.eq(recipe_id))
        .filter(recipe_ratings::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to check existing rating",
            "operation": "rate",
            "entity": "recipe_ratings",
            "recipe_id": recipe_id.to_string(),
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "check_exists"
        })))?;

    if let Some(model) = existing_rating {
        let mut active: recipe_ratings::ActiveModel = model.into();
        active.rating = Set(rating);
        active
            .update(db)
            .await
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to update recipe rating",
                "operation": "rate",
                "entity": "recipe_ratings",
                "recipe_id": recipe_id.to_string(),
                "user_id": user_id.to_string(),
                "rating": rating,
                "error": e.to_string(),
                "stage": "update"
            })))?;
    } else {
        recipe_ratings::ActiveModel {
            recipe_id: Set(recipe_id),
            user_id: Set(user_id),
            rating: Set(rating),
            ..Default::default()
        }
            .insert(db)
            .await
            .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to insert new recipe rating",
            "operation": "rate",
            "entity": "recipe_ratings",
            "recipe_id": recipe_id.to_string(),
            "user_id": user_id.to_string(),
            "rating": rating,
            "error": e.to_string(),
            "stage": "insert"
        })))?;
    }

    Ok(())
}

pub async fn unrate(db: &DatabaseConnection, recipe_id: Uuid, user_id: Uuid) -> Result<(), Error> {
    recipe_ratings::Entity::delete_many()
        .filter(recipe_ratings::Column::RecipeId.eq(recipe_id))
        .filter(recipe_ratings::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to delete recipe rating",
            "operation": "unrate",
            "entity": "recipe_ratings",
            "recipe_id": recipe_id.to_string(),
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "delete"
        })))?;

    Ok(())
}

pub async fn get_rating(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<RecipeRatingDto, Error> {
    #[derive(FromQueryResult)]
    struct Aggregates {
        avg_rating: Option<f64>,
        count: i64,
    }

    let stats = recipe_ratings::Entity::find()
        .select_only()
        .column_as(recipe_ratings::Column::Rating.avg(), "avg_rating")
        .column_as(recipe_ratings::Column::Rating.count(), "count")
        .filter(recipe_ratings::Column::RecipeId.eq(recipe_id))
        .into_model::<Aggregates>()
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch recipe rating statistics",
            "operation": "get_rating",
            "entity": "recipe_ratings",
            "recipe_id": recipe_id.to_string(),
            "error": e.to_string(),
            "stage": "aggregation_query"
        })))?
        .unwrap_or(Aggregates {
            avg_rating: None,
            count: 0,
        });

    let mut user_rating = None;
    if let Some(uid) = user_id {
        let personal_rating = recipe_ratings::Entity::find_by_id((recipe_id, uid))
            .one(db)
            .await
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to fetch user's rating",
                "operation": "get_rating",
                "entity": "recipe_ratings",
                "recipe_id": recipe_id.to_string(),
                "user_id": uid.to_string(),
                "error": e.to_string(),
                "stage": "user_rating_query"
            })))?;

        if let Some(m) = personal_rating {
            user_rating = Some(m.rating);
        }
    }

    Ok(RecipeRatingDto {
        average: stats.avg_rating.unwrap_or(0.0) as f32,
        count: stats.count,
        user_rating,
    })
}

pub async fn get_comment(db: &DatabaseConnection, comment_id: Uuid) -> Result<CommentDto, Error> {
    let res = recipe_comments::Entity::find_by_id(comment_id)
        .find_also_related(users::Entity)
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch comment",
            "operation": "get_comment",
            "entity": "recipe_comments",
            "comment_id": comment_id.to_string(),
            "error": e.to_string(),
            "stage": "fetch"
        })))?;

    if let Some((comment, user_opt)) = res {
        let username = user_opt
            .map(|u| u.username)
            .unwrap_or_else(|| "Deleted User".to_string());

        Ok(CommentDto {
            id: comment.id,
            recipe_id: comment.recipe_id,
            user_id: comment.user_id.unwrap_or_else(Uuid::nil),
            username,
            parent_id: comment.parent_id,
            content: if comment.deleted_at.is_some() {
                "This comment has been deleted.".to_string()
            } else {
                comment.content
            },
            created_at: comment.created_at.with_timezone(&Utc),
            edited_at: comment.edited_at.map(|dt| dt.with_timezone(&Utc)),
            children: Vec::new(),
            deleted_at: comment.deleted_at.map(|dt| dt.with_timezone(&Utc)),
        })
    } else {
        Err(Error::InternalServerError(json!({
            "message": "Comment not found in database",
            "operation": "get_comment",
            "entity": "recipe_comments",
            "comment_id": comment_id.to_string(),
            "stage": "validation"
        })))
    }
}

pub async fn get_comments(
    db: &DatabaseConnection,
    recipe_id: Uuid,
) -> Result<Vec<CommentDto>, Error> {
    let results = recipe_comments::Entity::find()
        .filter(recipe_comments::Column::RecipeId.eq(recipe_id))
        .find_also_related(users::Entity)
        .order_by_asc(recipe_comments::Column::CreatedAt)
        .all(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch recipe comments",
            "operation": "get_comments",
            "entity": "recipe_comments",
            "recipe_id": recipe_id.to_string(),
            "error": e.to_string(),
            "stage": "fetch"
        })))?;

    let all_comments: Vec<CommentDto> = results
        .into_iter()
        .map(|(comment, user_opt)| {
            let username = user_opt
                .map(|u| u.username)
                .unwrap_or_else(|| "Deleted User".to_string());

            CommentDto {
                id: comment.id,
                recipe_id: comment.recipe_id,
                user_id: comment.user_id.unwrap_or_else(Uuid::nil),
                username,
                parent_id: comment.parent_id,
                content: if comment.deleted_at.is_some() {
                    "This comment has been deleted.".to_string()
                } else {
                    comment.content
                },
                created_at: comment.created_at.with_timezone(&Utc),
                edited_at: Default::default(),
                children: Vec::new(),
                deleted_at: None,
            }
        })
        .collect();

    let mut children_map: HashMap<Uuid, Vec<CommentDto>> = HashMap::new();
    let mut root_comments = Vec::new();

    for comment in all_comments {
        if let Some(p_id) = comment.parent_id {
            children_map.entry(p_id).or_default().push(comment);
        } else {
            root_comments.push(comment);
        }
    }

    fn nest_replies(parent: &mut CommentDto, map: &mut HashMap<Uuid, Vec<CommentDto>>) {
        if let Some(children) = map.remove(&parent.id) {
            parent.children = children;
            for reply in &mut parent.children {
                nest_replies(reply, map);
            }
        }
    }

    for root in &mut root_comments {
        nest_replies(root, &mut children_map);
    }

    Ok(root_comments)
}

pub async fn add_comment(
    db: &DatabaseConnection,
    new_comment: CreateCommentDto,
    recipe_id: Uuid,
    user_id: Uuid,
) -> Result<CommentDto, Error> {
    let res = recipe_comments::ActiveModel {
        id: Set(Uuid::new_v4()),
        recipe_id: Set(recipe_id),
        user_id: Set(Some(user_id)),
        parent_id: Set(new_comment.parent_id),
        content: Set(new_comment.content.clone()),
        created_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    }
        .insert(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to insert comment",
        "operation": "add_comment",
        "entity": "recipe_comments",
        "recipe_id": recipe_id.to_string(),
        "user_id": user_id.to_string(),
        "parent_id": new_comment.parent_id.map(|p| p.to_string()),
        "error": e.to_string(),
        "stage": "insert"
    })))?;

    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch user for comment",
            "operation": "add_comment",
            "entity": "users",
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "user_fetch"
        })))?;

    let username = user
        .map(|u| u.username)
        .unwrap_or_else(|| "Unknown User".to_string());

    Ok(CommentDto {
        id: res.id,
        recipe_id: res.recipe_id,
        user_id: res.user_id.unwrap_or(user_id),
        username,
        parent_id: res.parent_id,
        content: res.content,
        created_at: res.created_at.with_timezone(&Utc),
        edited_at: res.edited_at.map(|dt| dt.with_timezone(&Utc)),
        children: Vec::new(),
        deleted_at: res.deleted_at.map(|dt| dt.with_timezone(&Utc)),
    })
}

pub async fn delete_comment(
    db: &DatabaseConnection,
    comment_id: Uuid,
) -> Result<CommentDto, Error> {
    let res = recipe_comments::Entity::find_by_id(comment_id)
        .find_also_related(users::Entity)
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch comment for deletion",
            "operation": "delete_comment",
            "entity": "recipe_comments",
            "comment_id": comment_id.to_string(),
            "error": e.to_string(),
            "stage": "fetch"
        })))?;

    if let Some((model, user_opt)) = res {
        let mut active: recipe_comments::ActiveModel = model.clone().into();
        active.deleted_at = Set(Some(Utc::now().into()));

        let updated_model = active
            .update(db)
            .await
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to mark comment as deleted",
                "operation": "delete_comment",
                "entity": "recipe_comments",
                "comment_id": comment_id.to_string(),
                "error": e.to_string(),
                "stage": "update"
            })))?;

        let username = user_opt
            .map(|u| u.username)
            .unwrap_or_else(|| "Deleted User".to_string());

        Ok(CommentDto {
            id: updated_model.id,
            recipe_id: updated_model.recipe_id,
            user_id: updated_model.user_id.unwrap_or_else(Uuid::nil),
            username,
            parent_id: updated_model.parent_id,
            content: "".to_string(),
            created_at: updated_model.created_at.with_timezone(&Utc),
            edited_at: updated_model.edited_at.map(|dt| dt.with_timezone(&Utc)),
            children: Vec::new(),
            deleted_at: updated_model.deleted_at.map(|dt| dt.with_timezone(&Utc)),
        })
    } else {
        Err(Error::InternalServerError(json!({
            "message": "Comment not found for deletion",
            "operation": "delete_comment",
            "entity": "recipe_comments",
            "comment_id": comment_id.to_string(),
            "stage": "validation"
        })))
    }
}

pub async fn edit_comment(
    db: &DatabaseConnection,
    comment_id: Uuid,
    new_comment: CommentDto,
) -> Result<CommentDto, Error> {
    let res = recipe_comments::Entity::find_by_id(comment_id)
        .find_also_related(users::Entity)
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch comment for editing",
            "operation": "edit_comment",
            "entity": "recipe_comments",
            "comment_id": comment_id.to_string(),
            "error": e.to_string(),
            "stage": "fetch"
        })))?;

    if let Some((model, user_opt)) = res {
        let mut active: recipe_comments::ActiveModel = model.into();
        active.content = Set(new_comment.content.clone());
        active.edited_at = Set(Some(chrono::Utc::now().into()));

        let updated_model = active
            .update(db)
            .await
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to update comment",
                "operation": "edit_comment",
                "entity": "recipe_comments",
                "comment_id": comment_id.to_string(),
                "error": e.to_string(),
                "stage": "update"
            })))?;

        let username = user_opt
            .map(|u| u.username)
            .unwrap_or_else(|| "Deleted User".to_string());

        Ok(CommentDto {
            id: updated_model.id,
            recipe_id: updated_model.recipe_id,
            user_id: updated_model.user_id.unwrap_or_else(Uuid::nil),
            username,
            parent_id: updated_model.parent_id,
            content: updated_model.content,
            created_at: updated_model.created_at.with_timezone(&Utc),
            edited_at: updated_model.edited_at.map(|dt| dt.with_timezone(&Utc)),
            children: Vec::new(),
            deleted_at: updated_model.deleted_at.map(|dt| dt.with_timezone(&Utc)),
        })
    } else {
        Err(Error::InternalServerError(json!({
            "message": "Comment not found for editing",
            "operation": "edit_comment",
            "entity": "recipe_comments",
            "comment_id": comment_id.to_string(),
            "stage": "validation"
        })))
    }
}