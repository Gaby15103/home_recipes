use crate::dto::ingredient_dto::{IngredientInput, IngredientRecipeViewDto, IngredientViewDto};
use crate::dto::unit_dto::UnitDto;
use crate::errors::Error;
use entity::{ingredient_translations, ingredient_units, ingredients};
use sea_orm::QueryFilter;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    QuerySelect, Set,
};
use serde_json::json;
use uuid::Uuid;

pub async fn create_and_link(
    txn: &DatabaseTransaction,
    group_id: Uuid,
    input: IngredientInput,
    lang: &str,
) -> Result<IngredientRecipeViewDto, crate::errors::Error> {
    let ingredient = ingredients::ActiveModel {
        id: Set(Uuid::new_v4()),
        ingredient_group_id: Set(group_id),
        quantity: Set(input.quantity),
        unit_id: Set(input.unit_id),
        position: Set(input.position),
        ..Default::default()
    }
        .insert(txn)
        .await
        .map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to insert ingredient into database",
        "operation": "create_and_link",
        "group_id": group_id.to_string(),
        "quantity": input.quantity,
        "unit_id": input.unit_id,
        "position": input.position,
        "error": e.to_string(),
        "stage": "ingredient_insert"
    })))?;

    let mut display_name = String::new();
    let mut display_note = None;

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
            .await
            .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to insert ingredient translation",
            "operation": "create_and_link",
            "ingredient_id": ingredient.id.to_string(),
            "language_code": &trans.language_code,
            "data": &trans.data,
            "error": e.to_string(),
            "stage": "translation_insert"
        })))?;

        if trans.language_code == lang {
            display_name = trans.data;
            display_note = trans.note;
        }
    }

    let unit = ingredient_units::Entity::find_by_id(ingredient.unit_id)
        .one(txn)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to query ingredient_units table",
            "operation": "create_and_link",
            "unit_id": ingredient.unit_id,
            "error": e.to_string(),
            "stage": "unit_lookup"
        })))?
        .ok_or_else(|| Error::InternalServerError(json!({
            "message": "Unit not found in database",
            "operation": "create_and_link",
            "unit_id": ingredient.unit_id,
            "ingredient_id": ingredient.id.to_string(),
            "error": "Unit ID does not exist",
            "stage": "unit_validation"
        })))?;

    Ok(IngredientRecipeViewDto {
        id: ingredient.id,
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
    let mut trans_query =
        ingredient_translations::Entity::find().find_also_related(ingredients::Entity);

    if let Some(s) = search.as_ref().filter(|s| !s.trim().is_empty()) {
        let pattern = format!("%{}%", s);
        trans_query = trans_query.filter(ingredient_translations::Column::Data.ilike(pattern));
    }

    let results = trans_query
        .limit(limit as u64)
        .all(db)
        .await
        .map_err(|e| {
            log::error!("Search Query Error: {:?}", e);
            Error::InternalServerError(json!({
                "message": "Failed to search ingredients",
                "operation": "get_all",
                "search_term": search.as_deref().unwrap_or(""),
                "limit": limit,
                "language_code": lang_code,
                "error": e.to_string(),
                "stage": "search_query"
            }))
        })?;

    if results.is_empty() {
        return Ok(vec![]);
    }

    let ingredient_ids: Vec<Uuid> = results
        .iter()
        .filter_map(|(_, ing)| ing.as_ref().map(|i| i.id))
        .collect();

    let full_data = ingredients::Entity::find()
        .filter(ingredients::Column::Id.is_in(ingredient_ids.clone()))
        .find_with_related(ingredient_translations::Entity)
        .all(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch full ingredient data with translations",
            "operation": "get_all",
            "ingredient_ids_count": ingredient_ids.len(),
            "language_code": lang_code,
            "error": e.to_string(),
            "stage": "full_data_fetch"
        })))?;

    let dto_list = full_data
        .into_iter()
        .map(|(ing, translations)| {
            let name = translations
                .iter()
                .find(|t| t.language_code == lang_code)
                .or_else(|| {
                    let def = ing.default_language.as_deref().unwrap_or("en");
                    translations.iter().find(|t| t.language_code == def)
                })
                .or_else(|| translations.first())
                .map(|t| t.data.clone())
                .unwrap_or_else(|| {
                    log::warn!(
                        "No translation found for ingredient {}, using default name",
                        ing.id
                    );
                    "Unknown".to_string()
                });

            IngredientViewDto { id: ing.id, name }
        })
        .collect();

    Ok(dto_list)
}