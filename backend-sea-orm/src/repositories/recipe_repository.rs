use crate::dto::ingredient_group_dto::IngredientGroupViewDto;
use crate::dto::recipe_dto::{CreateRecipeInput, GetFilter, RecipeViewDto};
use crate::dto::step_group_dto::StepGroupViewDto;
use crate::dto::tag_dto::{TagDto};
use crate::errors::Error;
use crate::repositories::{ingredient_group_repository, step_group_repository, tag_repository};
use entity::{ingredient_groups, ingredient_translations, ingredients, recipe_ingredients, recipe_translations, recipes};
use sea_orm::{ExprTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, Set, TransactionTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;
use std::ops::Deref;
use uuid::Uuid;
use migration::JoinType;

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<recipes::Model>, Error> {
    recipes::Entity::find().all(db).await.map_err(Error::from)
}
pub async fn find_by_query(
    db: &DatabaseConnection,
    filter: GetFilter,
    lang_code: &str
)-> Result<Option<Vec<recipes::Model>>, Error> {
    let mut query = recipes::Entity::find();

    // 2. Scope Filter
    query = query.filter(recipes::Column::IsPrivate.eq(filter.scope));

    // 3. The Search Text (Title & Description)
    if let Some(search_text) = filter.search {
        let pattern = format!("%{}%", search_text);
        query = query.filter(
            recipe_translations::Column::Title.like(&pattern)
                .or(recipe_translations::Column::Description.like(&pattern))
        );
    }

    // 4. Time range filters (Directly on recipes table)
    if let Some(max) = filter.max_prep {
        query = query.filter(recipes::Column::PrepTimeMinutes.lte(max));
    }
    if let Some(max) = filter.max_cook {
        query = query.filter(recipes::Column::CookTimeMinutes.lte(max));
    }

    // 5. Ingredient Search (Joining master ingredients and translations)
    if let Some(ing_search) = filter.ingredient {
        let pattern = format!("%{}%", ing_search);
        query = query
            .join(JoinType::InnerJoin, recipes::Relation::IngredientGroups.def())
            .join(JoinType::InnerJoin, ingredient_groups::Relation::RecipeIngredients.def())
            .join(JoinType::InnerJoin, recipe_ingredients::Relation::Ingredients.def())
            .join(JoinType::InnerJoin, ingredients::Relation::IngredientTranslations.def())
            // Match ingredient name in the SAME language as the search
            .filter(ingredient_translations::Column::LanguageCode.eq(lang_code))
            .filter(ingredient_translations::Column::Name.like(pattern));
    }

    // 6. Final cleanup: Ensure one result per recipe
    query = query.group_by(recipes::Column::Id)
        .order_by_desc(recipes::Column::CreatedAt);

    let results = query.all(db).await?;
    Ok(Option::from(results))
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
                author_id: Set(Some(new_recipe.author_id)),
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
                .or_else(|| translations.iter().find(|t| t.language_code == recipe_model.original_language_code))
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
