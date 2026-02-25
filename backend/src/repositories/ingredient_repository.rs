use sea_orm::{QueryFilter};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QuerySelect, Set};
use serde_json::json;
use uuid::Uuid;
use entity::{ ingredient_translations, ingredient_units, ingredients};
use crate::dto::ingredient_dto::{IngredientInput, IngredientRecipeViewDto, IngredientViewDto};
use crate::dto::unit_dto::UnitDto;
use crate::errors::Error;

pub async fn create_and_link(
    txn: &DatabaseTransaction,
    group_id: Uuid,
    input: IngredientInput,
    lang: &str,
) -> Result<IngredientRecipeViewDto, crate::errors::Error> {
    // 1. Create the Ingredient (The line item)
    let ingredient = ingredients::ActiveModel {
        id: Set(Uuid::new_v4()),
        ingredient_group_id: Set(group_id),
        quantity: Set(input.quantity),
        unit_id: Set(input.unit_id),
        position: Set(input.position),
        ..Default::default()
    }
        .insert(txn)
        .await?;

    // 2. Insert Translations (Unified: includes Name and Note)
    let mut display_name = String::new();
    let mut display_note = None;

    // We assume the frontend sends matched translations (name + note for same lang)
    // If translations and notes come in separate vectors in your Input DTO,
    // you'll need to zip/match them by language_code here.
    for trans in input.translations {

        ingredient_translations::ActiveModel {
            id: Set(Uuid::new_v4()),
            ingredient_id: Set(ingredient.id),
            language_code: Set(trans.language_code.clone()),
            data: Set(trans.data.clone()),
            note: Set(trans.note.clone()),
            ..Default::default()
        }
            .insert(txn)
            .await?;

        if trans.language_code == lang {
            display_name = trans.data;
            display_note = trans.note;
        }
    }

    // 3. Get Unit for DTO
    let unit = ingredient_units::Entity::find_by_id(ingredient.unit_id)
        .one(txn)
        .await?
        .ok_or(Error::NotFound(json!({"error": "Unit not found"})))?;

    Ok(IngredientRecipeViewDto {
        id: ingredient.id,
        ingredient_id: ingredient.id,
        name: display_name,
        quantity: ingredient.quantity,
        unit: UnitDto::from(unit),
        note: Option::from(display_note),
        position: ingredient.position,
    })
}
pub async fn get_all(
    db: &DatabaseConnection,
    search: Option<String>,
    limit: i32,
    lang_code: &str,
) -> Result<Vec<IngredientViewDto>, Error> {
    // 1. Start by finding Translations that match the search
    let mut trans_query = ingredient_translations::Entity::find()
        .find_also_related(ingredients::Entity);

    // 2. Filter by search text (if provided)
    if let Some(s) = search.as_ref().filter(|s| !s.trim().is_empty()) {
        let pattern = format!("%{}%", s);
        trans_query = trans_query.filter(ingredient_translations::Column::Data.ilike(pattern));
    }

    // 3. Execute query to get unique Ingredients
    // We limit and fetch to find the "Top 25" matching ingredients
    let results = trans_query
        .limit(limit as u64)
        .all(db)
        .await
        .map_err(|e| {
            eprintln!("Search Query Error: {:?}", e);
            Error::InternalServerError
        })?;

    if results.is_empty() {
        return Ok(vec![]);
    }

    // Collect the IDs of ingredients we found
    let ingredient_ids: Vec<Uuid> = results
        .iter()
        .filter_map(|(_, ing)| ing.as_ref().map(|i| i.id))
        .collect();

    // 4. Fetch the FULL data for these ingredients (all translations for fallback)
    // find_with_related gets us (Ingredient, Vec<Translations>)
    let full_data = ingredients::Entity::find()
        .filter(ingredients::Column::Id.is_in(ingredient_ids))
        .find_with_related(ingredient_translations::Entity)
        .all(db)
        .await?;

    // 5. Map to DTO with Fallback
    let dto_list = full_data
        .into_iter()
        .map(|(ing, translations)| {
            let name = translations.iter()
                .find(|t| t.language_code == lang_code)
                .or_else(|| {
                    let def = ing.default_language.as_deref().unwrap_or("en");
                    translations.iter().find(|t| t.language_code == def)
                })
                .or_else(|| translations.first())
                .map(|t| t.data.clone())
                .unwrap_or_else(|| "Unknown".to_string());

            IngredientViewDto {
                id: ing.id,
                name,
            }
        })
        .collect();

    Ok(dto_list)
}