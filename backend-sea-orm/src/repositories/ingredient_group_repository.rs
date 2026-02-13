use sea_orm::{DatabaseConnection, QueryFilter, QueryOrder};
use std::collections::HashMap;
use std::str::FromStr;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, Set};
use uuid::Uuid;
use entity::{ingredient_group_translations, ingredient_groups, ingredient_translations, ingredients, recipe_ingredient_translations, recipe_ingredients};
use crate::dto::ingredient_dto::IngredientViewDto;
use crate::dto::ingredient_group_dto::{IngredientGroupInput, IngredientGroupViewDto};
use crate::errors::Error;
use crate::repositories::ingredient_repository;
use crate::utils::unit::IngredientUnit;

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
        title: display_name,
        recipe_id: group.recipe_id,
        position: group.position,
        ingredients: ingredient_view_dtos,
    })
}
pub async fn find_by_recipe(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    lang: &str,
    default_lang_code: &str,
) -> Result<Vec<IngredientGroupViewDto>, Error> {
    // 1. Fetch groups
    let groups_with_trans = ingredient_groups::Entity::find()
        .filter(ingredient_groups::Column::RecipeId.eq(recipe_id))
        .order_by_asc(ingredient_groups::Column::Position)
        .find_with_related(ingredient_group_translations::Entity)
        .all(db)
        .await?;

    if groups_with_trans.is_empty() { return Ok(Vec::new()); }
    let group_ids: Vec<Uuid> = groups_with_trans.iter().map(|(g, _)| g.id).collect();

    // 2. Fetch notes - Include BOTH requested and default language
    let recipe_ingredients_with_notes = recipe_ingredients::Entity::find()
        .filter(recipe_ingredients::Column::IngredientGroupId.is_in(group_ids))
        .order_by_asc(recipe_ingredients::Column::Position)
        .find_with_related(recipe_ingredient_translations::Entity)
        // Only fetch translations for the two languages we care about
        .filter(
            recipe_ingredient_translations::Column::LanguageCode.is_in(vec![lang, default_lang_code])
        )
        .all(db)
        .await?;

    // 3. Fetch master names - Include BOTH requested and default language
    let master_ids: Vec<Uuid> = recipe_ingredients_with_notes.iter().map(|(ri, _)| ri.ingredient_id).collect();

    let name_translations = ingredient_translations::Entity::find()
        .filter(ingredient_translations::Column::IngredientId.is_in(master_ids))
        .filter(
            ingredient_translations::Column::LanguageCode.is_in(vec![lang, default_lang_code])
        )
        .all(db)
        .await?;

    // 4. Map everything into IngredientViewDtos
    let mut ingredients_map: HashMap<Uuid, Vec<IngredientViewDto>> = HashMap::new();

    for (rel, notes) in recipe_ingredients_with_notes {
        // Find best name from name_translations vector
        let name = name_translations.iter()
            .filter(|t| t.ingredient_id == rel.ingredient_id)
            .find(|t| t.language_code == lang) // Try preferred
            .or_else(|| name_translations.iter().filter(|t| t.ingredient_id == rel.ingredient_id).find(|t| t.language_code == default_lang_code)) // Try default
            .map(|t| t.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        // Find best note from the notes vector attached to this specific relation
        let note = notes.iter()
            .find(|t| t.language_code == lang)
            .or_else(|| notes.iter().find(|t| t.language_code == default_lang_code))
            .and_then(|t| t.note.clone());

        let unit = IngredientUnit::from_str(&rel.unit).unwrap_or(IngredientUnit::Gram);

        ingredients_map.entry(rel.ingredient_group_id).or_default().push(IngredientViewDto {
            id: rel.ingredient_id,
            name,
            quantity: rel.quantity,
            note,
            unit,
            position: rel.position,
        });
    }

    // 5. Final Assembly with Fallback for Group Titles
    let results = groups_with_trans.into_iter().map(|(group, trans)| {
        let title = trans.iter()
            .find(|t| t.language_code == lang)
            .or_else(|| trans.iter().find(|t| t.language_code == default_lang_code))
            .map(|t| t.title.clone())
            .unwrap_or_else(|| "Ingredients".to_string());

        let gid = group.id;
        IngredientGroupViewDto {
            id: gid,
            title,
            recipe_id: group.recipe_id,
            position: group.position,
            ingredients: ingredients_map.remove(&gid).unwrap_or_default(),
        }
    }).collect();

    Ok(results)
}