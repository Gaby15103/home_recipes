use std::ops::Deref;
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use crate::dto::recipe_dto::{CreateRecipeInput, RecipeDto, RecipeViewDto};
use crate::errors::Error;
use crate::repositories::{recipe_repository, recipe_translation_repository};

pub async fn get_all(
    db: &DatabaseConnection,
    lang_code: &str,
) -> Result<Vec<RecipeViewDto>, Error> {

    // Get all recipes
    let recipes = recipe_repository::find_all(db).await?;

    // For each recipe, get the translation
    let mut dtos = Vec::with_capacity(recipes.len());

    for recipe in recipes {
        // fetch translation (requested lang, fallback if missing)
        let translation = recipe_translation_repository::find_by_recipe_and_lang(
            db,
            recipe.id,
            lang_code,
            recipe.original_language_code.deref(),
        ).await?;

        // convert to DTO using your From impl
        let dto = RecipeViewDto::from((recipe, translation));
        dtos.push(dto);
    }

    Ok(dtos)
}

pub async fn get_by_id(
    db: &DatabaseConnection,
    id: Uuid,
    lang_code: &str,
) -> Result<RecipeViewDto, Error> {

    let recipe = recipe_repository::find_by_id(db, id).await?;
    let recipe_translation = recipe_translation_repository::find_by_recipe_and_lang(db, recipe.id, lang_code, recipe.original_language_code.deref()).await?;

    Ok(RecipeViewDto::from((recipe, recipe_translation)))
}

pub async fn create(
    db: &DatabaseConnection,
    new_recipe: CreateRecipeInput,
    preferred_language: &str,
) -> Result<RecipeViewDto, Error> {
    let inserted_recipe: RecipeViewDto = recipe_repository::create(db, new_recipe, preferred_language).await?;
    Ok(inserted_recipe)
}