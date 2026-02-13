// crate::repository::ingredient_repository

use std::ops::Deref;
use std::str::FromStr;
use sea_orm::{ActiveModelTrait, DatabaseTransaction, Set};
use uuid::Uuid;
use entity::{ingredient_translations, ingredients, recipe_ingredient_translations, recipe_ingredients};
use crate::dto::ingredient_dto::{IngredientInput, IngredientViewDto};
use crate::utils::unit::IngredientUnit;

pub async fn create_and_link(
    txn: &DatabaseTransaction,
    group_id: Uuid,
    input: IngredientInput,
    lang: &str,
) -> Result<IngredientViewDto, crate::errors::Error> {
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
    Ok(IngredientViewDto {
        id: master_ingredient.id,
        name: display_name,
        quantity: link.quantity,
        unit: IngredientUnit::from_str(link.unit.deref()).unwrap(),
        note: display_note.expect("REASON"),
        position: link.position,
    })
}