// crate::repository::ingredient_repository

use sea_orm::{QueryFilter, QueryOrder};
use std::ops::Deref;
use std::str::FromStr;
use actix_web::web::BufMut;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QuerySelect, RelationTrait, Set};
use uuid::Uuid;
use entity::{ingredient_groups, ingredient_translations, ingredients, recipe_ingredient_translations, recipe_ingredients, recipes};
use migration::JoinType;
use crate::dto::ingredient_dto::{IngredientInput, IngredientRecipeViewDto, IngredientViewDto};
use crate::errors::Error;
use crate::utils::unit::IngredientUnit;

pub async fn create_and_link(
    txn: &DatabaseTransaction,
    group_id: Uuid,
    input: IngredientInput,
    lang: &str,
) -> Result<IngredientRecipeViewDto, crate::errors::Error> {
    // 1. Create the Master Ingredient record
    // Note: If you want to prevent duplicates, implement the "Find or Create" logic here
    let master_ingredient = ingredients::ActiveModel {
        id: Set(Uuid::new_v4()),
        // Add any global fields here (e.g., category, icon)
        ..Default::default()
    }
        .insert(txn)
        .await?;

    // 2. Insert Master Ingredient Translations
    let mut display_name = String::new();
    for trans in input.translations {
        ingredient_translations::ActiveModel {
            ingredient_id: Set(master_ingredient.id),
            language_code: Set(trans.language.clone()),
            name: Set(trans.name.clone()),
            ..Default::default()
        }
            .insert(txn)
            .await?;

        if trans.language == lang {
            display_name = trans.name;
        }
    }

    // 3. Link to the Group (The Recipe-specific details)
    let link = recipe_ingredients::ActiveModel {
        id: Set(Uuid::new_v4()),
        ingredient_group_id: Set(group_id),
        ingredient_id: Set(master_ingredient.id),
        quantity: Set(input.quantity), // This is where rust_decimal::Decimal is used
        unit: Set(input.unit.clone()),
        position: Set(input.position),
        ..Default::default()
    }
        .insert(txn)
        .await?;

    // 4. Handle Note Translations (Specific to this recipe instance)
    let mut display_note = None;
    if let Some(notes) = input.note_translations {
        for note_input in notes {
            recipe_ingredient_translations::ActiveModel {
                recipe_ingredient_id: Set(link.id),
                language_code: Set(note_input.language_code.clone()),
                note: Set(note_input.note.clone()),
                ..Default::default()
            }
                .insert(txn)
                .await?;

            if note_input.language_code == lang {
                display_note = Some(note_input.note);
            }
        }
    }

    // 5. Return the View DTO
    Ok(IngredientRecipeViewDto {
        id: master_ingredient.id,
        name: display_name,
        quantity: link.quantity,
        unit: IngredientUnit::from_str(link.unit.deref()).unwrap(),
        note: display_note.expect("REASON"),
        position: link.position,
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
        trans_query = trans_query.filter(ingredient_translations::Column::Name.ilike(pattern));
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
                .map(|t| t.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());

            IngredientViewDto {
                id: ing.id,
                name,
            }
        })
        .collect();

    Ok(dto_list)
}