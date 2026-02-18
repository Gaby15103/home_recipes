use crate::dto::ingredient_group_dto::IngredientGroupViewDto;
use crate::dto::recipe_dto::{CreateRecipeInput, RecipeFilter, RecipeFilterByPage, RecipeViewDto};
use crate::dto::step_group_dto::StepGroupViewDto;
use crate::dto::tag_dto::TagDto;
use crate::errors::Error;
use crate::repositories::{ingredient_group_repository, step_group_repository, tag_repository};
use entity::{favorites, ingredient_groups, ingredient_translations, ingredients, recipe_ingredients, recipe_translations, recipes};
use migration::JoinType;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, DeleteResult, PaginatorTrait, Set, TransactionTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{ExprTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait};
use serde_json::json;
use std::ops::Deref;
use uuid::Uuid;

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<recipes::Model>, Error> {
    recipes::Entity::find().all(db).await.map_err(Error::from)
}
pub async fn find_by_query(
    db: &DatabaseConnection,
    filter: RecipeFilter,
    lang_code: &str,
) -> Result<Option<Vec<recipes::Model>>, Error> {
    let mut query = recipes::Entity::find();

    // 1. The Scope Logic Check
    // Assuming filter.scope = true means "Public" and IsPrivate = true means "Private"
    // Adjust the '!' based on your specific UI logic
    query = query.filter(recipes::Column::IsPrivate.eq(!filter.scope));

    // 2. Search Text (Only join and filter if search is NOT empty)
    if let Some(s) = &filter.search {
        if !s.trim().is_empty() {
            let pattern = format!("%{}%", s);
            query = query
                .join(JoinType::LeftJoin, recipes::Relation::RecipeTranslations.def())
                .filter(
                    recipe_translations::Column::LanguageCode.eq(lang_code)
                        .and(recipe_translations::Column::Title.like(&pattern)
                            .or(recipe_translations::Column::Description.like(&pattern)))
                );
        }
    }

    // 3. Ingredient Search (Only join if search is NOT empty)
    // Using InnerJoin here is fine ONLY if a search term exists.
    if let Some(i) = &filter.ingredient {
        if !i.trim().is_empty() {
            let pattern = format!("%{}%", i);
            query = query
                .join(JoinType::InnerJoin, recipes::Relation::IngredientGroups.def())
                .join(JoinType::InnerJoin, ingredient_groups::Relation::RecipeIngredients.def())
                .join(JoinType::InnerJoin, recipe_ingredients::Relation::Ingredients.def())
                .join(JoinType::InnerJoin, ingredients::Relation::IngredientTranslations.def())
                .filter(ingredient_translations::Column::LanguageCode.eq(lang_code))
                .filter(ingredient_translations::Column::Name.like(pattern));
        }
    }

    // 4. Grouping & Execution
    // Grouping by ID is vital to avoid duplicates from the joins
    query = query
        .group_by(recipes::Column::Id)
        .order_by_desc(recipes::Column::CreatedAt);

    let results = query.all(db).await
        .map_err(|e| {
            eprintln!("Database Error: {:?}", e);
            Error::InternalServerError
        })?;

    if results.is_empty() {
        Ok(None)
    } else {
        Ok(Some(results))
    }
}

pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<recipes::Model, Error> {
    recipes::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(json!({"error":"Recipe not found"})))
}
pub async fn create(
    db: &DatabaseConnection,
    new_recipe: CreateRecipeInput,
    preferred_language: &str,
) -> Result<RecipeViewDto, Error> {
    let pref_lang = preferred_language.to_string();
    db.transaction::<_, RecipeViewDto, Error>(|txn| {
        Box::pin(async move {
            // 1. Insert the Base Recipe
            let recipe_model = recipes::ActiveModel {
                image_url: Set(new_recipe.image_url),
                author_id: Set(new_recipe.author_id),
                author: Set(new_recipe.author),
                servings: Set(new_recipe.servings),
                prep_time_minutes: Set(new_recipe.prep_time_minutes),
                cook_time_minutes: Set(new_recipe.cook_time_minutes),
                is_private: Set(new_recipe.is_private),
                original_language_code: Set(new_recipe.primary_language),
                ..Default::default()
            }
            .insert(txn)
            .await?;

            // 2. Insert Translations
            for trans in new_recipe.translations {
                recipe_translations::ActiveModel {
                    recipe_id: Set(recipe_model.id),
                    language_code: Set(trans.language_code),
                    title: Set(trans.title),
                    description: Set(trans.description),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
            }
            let inserted_tags: Vec<TagDto> =
                tag_repository::find_or_create_tags(txn, new_recipe.tags, recipe_model.id).await?;
            let inserted_ingredient_group: Vec<IngredientGroupViewDto> =
                ingredient_group_repository::create_multiple(
                    &txn,
                    recipe_model.id.clone(),
                    new_recipe.ingredient_groups,
                    recipe_model.original_language_code.deref(),
                )
                .await?;
            let inserted_step_group: Vec<StepGroupViewDto> =
                step_group_repository::create_multiple(
                    &txn,
                    recipe_model.id.clone(),
                    new_recipe.step_groups,
                    recipe_model.original_language_code.deref(),
                )
                .await?;

            let translations = recipe_translations::Entity::find()
                .filter(recipe_translations::Column::RecipeId.eq(recipe_model.id))
                .all(txn)
                .await?;

            let main_trans = translations
                .iter()
                .find(|t| t.language_code == pref_lang)
                .or_else(|| {
                    translations
                        .iter()
                        .find(|t| t.language_code == recipe_model.original_language_code)
                })
                .cloned()
                .ok_or(Error::InternalServerError)?;

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
    .map_err(|e| e.into())
}
pub async fn find_by_query_by_page(
    db: &DatabaseConnection,
    filter: RecipeFilterByPage,
    lang_code: &str,
) -> Result<Option<Vec<recipes::Model>>, Error> {
    let mut query = recipes::Entity::find();

    if let Some(filter) = filter.filters {
        query = query.filter(recipes::Column::IsPrivate.eq(!filter.scope));
        if let Some(s) = &filter.search {
            if !s.trim().is_empty() {
                let pattern = format!("%{}%", s);
                query = query
                    .join(JoinType::LeftJoin, recipes::Relation::RecipeTranslations.def())
                    .filter(
                        recipe_translations::Column::LanguageCode.eq(lang_code)
                            .and(recipe_translations::Column::Title.like(&pattern)
                                .or(recipe_translations::Column::Description.like(&pattern)))
                    );
            }
        }

        if let Some(i) = &filter.ingredient {
            if !i.trim().is_empty() {
                let pattern = format!("%{}%", i);
                query = query
                    .join(JoinType::InnerJoin, recipes::Relation::IngredientGroups.def())
                    .join(JoinType::InnerJoin, ingredient_groups::Relation::RecipeIngredients.def())
                    .join(JoinType::InnerJoin, recipe_ingredients::Relation::Ingredients.def())
                    .join(JoinType::InnerJoin, ingredients::Relation::IngredientTranslations.def())
                    .filter(ingredient_translations::Column::LanguageCode.eq(lang_code))
                    .filter(ingredient_translations::Column::Name.like(pattern));
            }
        }
    }




    query = query
        .group_by(recipes::Column::Id)
        .order_by_desc(recipes::Column::CreatedAt);

    let page = filter.page.unwrap_or(1);
    let per_page = filter.per_page.unwrap_or(10);

    let paginator = query.paginate(db, per_page as u64);
    let results = paginator.fetch_page((page - 1) as u64).await?;

    if results.is_empty() {
        Ok(None)
    } else {
        Ok(Some(results))
    }
}

pub async fn delete(
    db: &DatabaseConnection,
    id: Uuid,
)-> Result<DeleteResult, Error> {
    let result = recipes::Entity::delete_by_id(id).exec(db).await?;
    Ok(result)
}
pub async fn get_favorites(
    db: &DatabaseConnection,
    user_id: Uuid,
)->Result<Vec<recipes::Model>,Error>{
    let recipes = recipes::Entity::find()
        .join(JoinType::InnerJoin, recipes::Relation::Favorites.def())
        .filter(favorites::Column::UserId.eq(user_id))
        .all(db)
        .await?;
    Ok(recipes)
}
pub async fn update(
    db: &DatabaseConnection,
    updated_recipe: CreateRecipeInput,
    recipe_id: Uuid,
    lang_code: &str,
) -> Result<RecipeViewDto, Error> {
    // FIX: Clone the borrowed reference into an owned String
    let lang_code_owned = lang_code.to_string();

    db.transaction::<_, RecipeViewDto, Error>(|txn| {
        // Clone for use inside the move block
        let lang_code = lang_code_owned.clone();

        Box::pin(async move {
            // 1. Fetch existing base recipe
            let existing = recipes::Entity::find_by_id(recipe_id)
                .one(txn)
                .await?
                .ok_or(Error::NotFound(json!({"error": "Recipe not found"})))?;

            // 2. Base Recipe: Update ONLY if different
            let mut active_model: recipes::ActiveModel = existing.clone().into();
            let mut base_changed = false;

            if existing.image_url != updated_recipe.image_url {
                active_model.image_url = Set(updated_recipe.image_url);
                base_changed = true;
            }
            if existing.servings != updated_recipe.servings {
                active_model.servings = Set(updated_recipe.servings);
                base_changed = true;
            }
            if existing.prep_time_minutes != updated_recipe.prep_time_minutes {
                active_model.prep_time_minutes = Set(updated_recipe.prep_time_minutes);
                base_changed = true;
            }
            if existing.cook_time_minutes != updated_recipe.cook_time_minutes {
                active_model.cook_time_minutes = Set(updated_recipe.cook_time_minutes);
                base_changed = true;
            }
            if existing.is_private != updated_recipe.is_private {
                active_model.is_private = Set(updated_recipe.is_private);
                base_changed = true;
            }

            let recipe_model = if base_changed {
                active_model.update(txn).await?
            } else {
                existing
            };

            // 3. Translations: Sync and Cleanup
            // Collect incoming codes to know what to keep
            let incoming_codes: Vec<String> = updated_recipe.translations
                .iter()
                .map(|t| t.language_code.clone())
                .collect();

            // Delete translations no longer present in the input
            recipe_translations::Entity::delete_many()
                .filter(recipe_translations::Column::RecipeId.eq(recipe_id))
                .filter(recipe_translations::Column::LanguageCode.is_not_in(incoming_codes))
                .exec(txn)
                .await?;

            for trans_input in updated_recipe.translations {
                let existing_trans = recipe_translations::Entity::find()
                    .filter(recipe_translations::Column::RecipeId.eq(recipe_id))
                    .filter(recipe_translations::Column::LanguageCode.eq(&trans_input.language_code))
                    .one(txn)
                    .await?;

                match existing_trans {
                    Some(et) => {
                        if et.title != trans_input.title || et.description != trans_input.description {
                            let mut trans_active: recipe_translations::ActiveModel = et.into();
                            trans_active.title = Set(trans_input.title);
                            trans_active.description = Set(trans_input.description);
                            trans_active.update(txn).await?;
                        }
                    }
                    None => {
                        recipe_translations::ActiveModel {
                            recipe_id: Set(recipe_id),
                            language_code: Set(trans_input.language_code),
                            title: Set(trans_input.title),
                            description: Set(trans_input.description),
                            ..Default::default()
                        }
                            .insert(txn)
                            .await?;
                    }
                }
            }

            // 4. Sync Tags, Ingredients, and Steps
            let inserted_tags = tag_repository::find_or_create_tags(txn, updated_recipe.tags, recipe_id).await?;

            let inserted_ingredient_group = ingredient_group_repository::create_multiple(
                txn, recipe_id, updated_recipe.ingredient_groups, &recipe_model.original_language_code
            ).await?;

            let inserted_step_group = step_group_repository::create_multiple(
                txn, recipe_id, updated_recipe.step_groups, &recipe_model.original_language_code
            ).await?;

            // 5. Return the DTO using the now-owned lang_code
            let final_trans = recipe_translations::Entity::find()
                .filter(recipe_translations::Column::RecipeId.eq(recipe_id))
                .filter(recipe_translations::Column::LanguageCode.eq(&lang_code))
                .one(txn)
                .await?
                .ok_or(Error::InternalServerError)?;

            Ok(RecipeViewDto::build(
                recipe_model,
                final_trans,
                inserted_tags,
                inserted_ingredient_group,
                inserted_step_group,
            ))
        })
    })
        .await
        .map_err(|e| e.into())
}