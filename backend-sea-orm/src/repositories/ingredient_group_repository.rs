use sea_orm::{ActiveModelTrait, DatabaseTransaction, Set};
use uuid::Uuid;
use entity::{ingredient_group_translations, ingredient_groups, ingredient_translations, ingredients, recipe_ingredient_translations, recipe_ingredients};
use crate::dto::ingredient_group_dto::{IngredientGroupInput, IngredientGroupViewDto};
use crate::errors::Error;
use crate::repositories::ingredient_repository;

pub async fn create_multiple(
    txn: &DatabaseTransaction,
    recipe_id: Uuid,
    ingredient_group_input: Vec<IngredientGroupInput>,
    lang: &str,
)->Result<Vec<IngredientGroupViewDto>, Error>{
    let mut ingredient_groups: Vec<IngredientGroupViewDto> = Vec::new();
    for ingredient_group in ingredient_group_input {
        let inserted_ingredient_group = create(txn, recipe_id, ingredient_group,lang).await?;
        ingredient_groups.push(inserted_ingredient_group)
    }
    Ok(ingredient_groups)
}
pub async fn create(
    txn: &DatabaseTransaction, // Removed mut, Sea-ORM uses &DatabaseTransaction
    recipe_id: Uuid,
    ingredient_group_input: IngredientGroupInput,
    lang: &str, // Added target language for the ViewDto
) -> Result<IngredientGroupViewDto, Error> {
    // 1. Insert Group
    let group = ingredient_groups::ActiveModel {
        recipe_id: Set(recipe_id),
        position: Set(ingredient_group_input.position),
        ..Default::default()
    }
        .insert(txn)
        .await?;

    // 2. Insert Group Translations & pick the one for the ViewDto
    let mut display_name = String::new();
    for trans in ingredient_group_input.translations {
        let inserted_trans = ingredient_group_translations::ActiveModel {
            ingredient_group_id: Set(group.id),
            language_code: Set(trans.language_code.clone()),
            title: Set(trans.title.clone()),
            ..Default::default()
        }
            .insert(txn)
            .await?;

        if trans.language_code == lang {
            display_name = trans.title;
        }
    }

    // 3. Insert Ingredients and collect ViewDtos
    let mut ingredient_view_dtos = Vec::new();

    for ing_input in ingredient_group_input.ingredients {
        let ing_view = ingredient_repository::create_and_link(
            txn,
            group.id,
            ing_input,
            lang
        ).await?;

        ingredient_view_dtos.push(ing_view);
    }

    Ok(IngredientGroupViewDto {
        id: group.id,
        name: display_name,
        recipe_id: group.recipe_id,
        position: group.position,
        ingredients: ingredient_view_dtos,
    })
}