use crate::dto::recipe_dto::{CreateRecipeInput, EditRecipeInput, RecipeDto, RecipeEditorDto, RecipeFilter, RecipeFilterByPage, RecipeResponse, RecipeViewDto};
use crate::dto::tag_dto::TagDto;
use crate::errors::Error;
use crate::repositories::{ingredient_group_repository, recipe_repository, recipe_translation_repository, recipe_version_repository, step_group_repository, step_repository, tag_repository};
use crate::utils::file_upload::move_file_to_recipes;
use actix_web::HttpResponse;
use sea_orm::DatabaseConnection;
use std::fs;
use std::ops::Deref;
use actix::fut::ok;
use uuid::Uuid;
use crate::domain::user::{AuthenticatedUser, Role};
use crate::dto::comment_dto::{CommentDto, CreateCommentDto};
use crate::dto::user_dto::UserResponseDto;

pub async fn get_all(
    db: &DatabaseConnection,
    lang_code: &str,
    filter: RecipeFilter,
) -> Result<Vec<RecipeViewDto>, Error> {
    let recipes = recipe_repository::find_by_query(db, filter, lang_code).await?;

    let mut dtos = Vec::new();

    if let Some(recipes) = recipes {
        for recipe in recipes {
            // fetch translation (requested lang, fallback if missing)
            let translation = recipe_translation_repository::find_translation(
                db,
                recipe.id,
                lang_code,
                recipe.original_language_code.deref(),
            )
            .await?;
            let dto = RecipeViewDto::from((recipe, translation));
            dtos.push(dto);
        }
    }

    Ok(dtos)
}

pub async fn get_by_id(
    db: &DatabaseConnection,
    id: Uuid,
    lang_code: &str,
    include_translations: bool,
) -> Result<RecipeResponse, Error> {
    let recipe = recipe_repository::find_by_id(db, id).await?;
    let tags = tag_repository::find_by_recipe(db, recipe.id).await?;

    if include_translations {
        let all_translations = recipe_translation_repository::find_translations(db, id).await?;
        let step_groups = step_group_repository::find_all_by_recipe(db, id).await?;
        let ingredient_groups = ingredient_group_repository::find_all_by_recipe(db, id).await?;
        Ok(RecipeResponse::Editor(RecipeEditorDto::build_full(
            recipe,
            all_translations,
            tags,
            ingredient_groups,
            step_groups,
        )))
    } else {
        let recipe_translation = recipe_translation_repository::find_translation(
            db,
            recipe.id,
            lang_code,
            &recipe.original_language_code,
        )
        .await?;

        let step_groups = step_group_repository::find_by_recipe(
            db,
            recipe.id,
            lang_code,
            &recipe.original_language_code,
        )
        .await?;

        let ingredient_groups = ingredient_group_repository::find_by_recipe(
            db,
            recipe.id,
            lang_code,
            &recipe.original_language_code,
        )
        .await?;

        Ok(RecipeResponse::View(RecipeViewDto::build(
            recipe,
            recipe_translation,
            tags,
            ingredient_groups,
            step_groups,
        )))
    }
}

pub async fn create(
    db: &DatabaseConnection,
    mut new_recipe: CreateRecipeInput,
    preferred_language: &str,
) -> Result<RecipeViewDto, Error> {
    let target_dir = "assets/recipes";

    // 1. Ensure the destination directory exists
    fs::create_dir_all(target_dir)?;

    // 2. Move Main Recipe Image
    new_recipe.image_url = move_file_to_recipes(&new_recipe.image_url, target_dir)?;

    // 3. Move Step Images
    for group in &mut new_recipe.step_groups {
        for step in &mut group.steps {
            if let Some(temp_path) = &step.image_url {
                step.image_url = Some(move_file_to_recipes(temp_path, target_dir)?);
            }
        }
    }

    let inserted_recipe: RecipeViewDto =
        recipe_repository::create(db, new_recipe, preferred_language).await?;
    Ok(inserted_recipe)
}

pub async fn get_all_by_page(
    db: &DatabaseConnection,
    lang_code: &str,
    filter: RecipeFilterByPage,
) -> Result<Vec<RecipeViewDto>, Error> {
    let recipes = recipe_repository::find_by_query_by_page(db, filter, lang_code).await?;

    let mut dtos = Vec::new();

    if let Some(recipes) = recipes {
        for recipe in recipes {
            // fetch translation (requested lang, fallback if missing)
            let translation = recipe_translation_repository::find_translation(
                db,
                recipe.id,
                lang_code,
                recipe.original_language_code.deref(),
            )
            .await?;
            let dto = RecipeViewDto::from((recipe, translation));
            dtos.push(dto);
        }
    }
    Ok(dtos)
}
pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<bool, Error> {
    let result = recipe_repository::delete(db, id).await?;
    Ok(result.rows_affected > 0)
}
pub async fn update(
    db: &DatabaseConnection,
    updated_recipe: EditRecipeInput,
    recipe_id: Uuid,
    lang_code: &str,
    user: UserResponseDto
) -> Result<RecipeViewDto, Error> {
    let original = get_by_id(db, recipe_id, lang_code, true).await?;
    recipe_repository::update(db, updated_recipe, recipe_id, lang_code).await?;
    match original {
        RecipeResponse::View(_) =>{} ,
        RecipeResponse::Editor(original) => {
            recipe_version_repository::create(db, original, user.id).await?;

        }
    }
    let result = get_by_id(db, recipe_id, lang_code, false).await?;
    match result {
        RecipeResponse::View(recipe_view) =>{
            Ok(recipe_view)
        } ,
        RecipeResponse::Editor(_) => {
            Err(Error::InternalServerError)
        }
    }
}

pub async fn analytics(
    db: &DatabaseConnection,
    recipe_id: Uuid,
)->Result<u64, Error> {
    recipe_repository::get_analytics(db, recipe_id).await
}
pub async fn add_view(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Option<Uuid>,
)->Result<(), Error> {
    recipe_repository::add_view(db, recipe_id, user_id).await?;
    Ok(())
}
pub async fn toogle_favorite(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Uuid,
)->Result<bool, Error> {
    recipe_repository::toogle_favorite(db, recipe_id, user_id).await
}
pub async fn rate(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Uuid,
    rating: i32
)->Result<(), Error> {
    recipe_repository::rate(db,recipe_id, user_id, rating).await
}
pub async fn unrate(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Uuid,
)->Result<(), Error> {
    recipe_repository::unrate(db, recipe_id, user_id).await
}
pub async fn get_rating(
    db: &DatabaseConnection,
    recipe_id: Uuid,
)->Result<f32, Error> {
    recipe_repository::get_rating(db, recipe_id).await
}
pub async fn get_comments(
    db: &DatabaseConnection,
    recipe_id: Uuid,
)->Result<Vec<CommentDto>, Error> {
    recipe_repository::get_comments(db, recipe_id).await
}
pub async fn add_comment(
    db: &DatabaseConnection,
    new_comment: CreateCommentDto,
    recipe_id: Uuid,
    user_id: Uuid,
)->Result<CommentDto, Error> {
    recipe_repository::add_comment(db,new_comment, recipe_id, user_id).await
}
pub async fn delete_comment(
    db: &DatabaseConnection,
    comment_id: Uuid,
    auth: AuthenticatedUser
)->Result<CommentDto, Error> {
    let comment = recipe_repository::get_comment(db, comment_id).await?;
    auth.require_owner_or_roles(comment.user_id,&[Role::Admin,Role::Moderator,Role::Superuser])?;
    recipe_repository::delete_comment(db, comment_id).await
}