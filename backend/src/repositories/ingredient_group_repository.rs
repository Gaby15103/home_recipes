use sea_orm::{DatabaseConnection, QueryFilter, QueryOrder};
use std::collections::HashMap;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, Set};
use serde_json::json;
use uuid::Uuid;
use entity::{ingredient_group_translations, ingredient_groups, ingredient_translations, ingredients};
use crate::dto::ingredient_dto::{IngredientEditorDto, IngredientRecipeViewDto, IngredientTranslationsDto};
use crate::dto::ingredient_group_dto::{EditIngredientGroupInput, IngredientGroupEditorDto, IngredientGroupInput, IngredientGroupTranslationDto, IngredientGroupViewDto};
use crate::errors::Error;
use crate::repositories::{ingredient_repository, unit_repository};

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
    default_lang: &str,
) -> Result<Vec<IngredientGroupViewDto>, Error> {
    // 1. Fetch groups + group translations
    let groups_with_trans = ingredient_groups::Entity::find()
        .filter(ingredient_groups::Column::RecipeId.eq(recipe_id))
        .order_by_asc(ingredient_groups::Column::Position)
        .find_with_related(ingredient_group_translations::Entity)
        .all(db)
        .await?;

    if groups_with_trans.is_empty() { return Ok(vec![]); }
    let group_ids: Vec<Uuid> = groups_with_trans.iter().map(|(g, _)| g.id).collect();

    // 2. Fetch the actual Ingredients (the line items) for these groups
    let ingredients_list = ingredients::Entity::find()
        .filter(ingredients::Column::IngredientGroupId.is_in(group_ids))
        .order_by_asc(ingredients::Column::Position)
        .all(db)
        .await?;

    let ingredient_ids: Vec<Uuid> = ingredients_list.iter().map(|i| i.id).collect();

    // 3. Fetch Translations for those specific ingredients
    let translations = ingredient_translations::Entity::find()
        .filter(ingredient_translations::Column::IngredientId.is_in(ingredient_ids))
        .filter(ingredient_translations::Column::LanguageCode.is_in(vec![lang, default_lang]))
        .all(db)
        .await?;

    // 4. Map everything together
    let mut ingredients_map: HashMap<Uuid, Vec<IngredientRecipeViewDto>> = HashMap::new();

    for ing in ingredients_list {
        // Find the best translation for this specific ingredient
        let translation = translations.iter()
            .filter(|t| t.ingredient_id == ing.id)
            .find(|t| t.language_code == lang)
            .or_else(|| translations.iter().filter(|t| t.ingredient_id == ing.id).find(|t| t.language_code == default_lang));

        let unit = unit_repository::find_by_id(db, ing.unit_id).await?;

        // Fallback name if no translation exists at all
        let name = translation.map(|t| t.data.clone()).unwrap_or_else(|| "Unknown Ingredient".to_string());
        let note = translation.and_then(|t| t.note.clone());

        ingredients_map.entry(ing.ingredient_group_id).or_default().push(IngredientRecipeViewDto {
            id: ing.id, // This is the ingredient line-item ID
            ingredient_id: ing.id,
            name,
            quantity: ing.quantity,
            note,
            unit,
            position: ing.position,
        });
    }

    // 5. Final Assembly
    let results = groups_with_trans.into_iter().map(|(group, trans_list)| {
        let title = trans_list.iter()
            .find(|t| t.language_code == lang)
            .or_else(|| trans_list.iter().find(|t| t.language_code == default_lang))
            .map(|t| t.title.clone())
            .unwrap_or_else(|| "Ingredients".to_string());

        IngredientGroupViewDto {
            id: group.id,
            title,
            recipe_id: group.recipe_id,
            position: group.position,
            ingredients: ingredients_map.remove(&group.id).unwrap_or_default(),
        }
    }).collect();

    Ok(results)
}
pub async fn find_all_by_recipe(
    db: &DatabaseConnection,
    recipe_id: Uuid,
) -> Result<Vec<IngredientGroupEditorDto>, Error> {
    // 1. Fetch Groups + Group Title Translations
    let groups_with_trans = ingredient_groups::Entity::find()
        .filter(ingredient_groups::Column::RecipeId.eq(recipe_id))
        .order_by_asc(ingredient_groups::Column::Position)
        .find_with_related(ingredient_group_translations::Entity)
        .all(db)
        .await?;

    let group_ids: Vec<Uuid> = groups_with_trans.iter().map(|(g, _)| g.id).collect();
    if group_ids.is_empty() { return Ok(vec![]); }

    // 2. Fetch the Ingredients (the line items) for these groups
    let ingredients_list = ingredients::Entity::find()
        .filter(ingredients::Column::IngredientGroupId.is_in(group_ids))
        .order_by_asc(ingredients::Column::Position)
        .all(db)
        .await?;

    let ingredient_ids: Vec<Uuid> = ingredients_list.iter().map(|i| i.id).collect();

    // 3. Fetch ALL Translations (names and notes) for these ingredients
    let all_translations = ingredient_translations::Entity::find()
        .filter(ingredient_translations::Column::IngredientId.is_in(ingredient_ids))
        .all(db)
        .await?;

    // 4. Map everything together
    let mut ingredients_map: HashMap<Uuid, Vec<IngredientEditorDto>> = HashMap::new();

    for ing in ingredients_list {
        let ing_id = ing.id;

        // Collect all translations associated with this specific ingredient line-item
        let translations_for_this_ing = all_translations.iter()
            .filter(|t| t.ingredient_id == ing_id);

        let translations_dto: Vec<IngredientTranslationsDto> = translations_for_this_ing.clone()
            .map(|t| IngredientTranslationsDto {
                id: t.id,
                language_code: t.language_code.clone(),
                data: t.data.clone(),
                note: t.note.clone(),
            })
            .collect();

        let unit = unit_repository::find_by_id(db, ing.unit_id).await?;

        ingredients_map.entry(ing.ingredient_group_id).or_default().push(IngredientEditorDto {
            id: ing_id,
            ingredient_id: ing_id, // In this schema, the line item is the ingredient
            quantity: ing.quantity,
            position: ing.position,
            unit_id: ing.unit_id,
            unit,
            translations: translations_dto,
        });
    }

    // 5. Final Assembly
    let results = groups_with_trans.into_iter().map(|(group, trans_list)| {
        let gid = group.id;
        IngredientGroupEditorDto {
            id: gid,
            recipe_id: group.recipe_id,
            position: group.position,
            translations: trans_list.into_iter().map(|t| IngredientGroupTranslationDto {
                language_code: t.language_code,
                title: t.title,
            }).collect(),
            ingredients: ingredients_map.remove(&gid).unwrap_or_default(),
        }
    }).collect();

    Ok(results)
}
pub async fn update(
    txn: &DatabaseTransaction,
    recipe_id: Uuid,
    ingredient_group: Vec<EditIngredientGroupInput>,
) -> Result<(), Error> {
    let incoming_group_ids: Vec<Uuid> = ingredient_group.iter().filter_map(|g| g.id).collect();

    // 1. Delete groups no longer in the input
    ingredient_groups::Entity::delete_many()
        .filter(ingredient_groups::Column::RecipeId.eq(recipe_id))
        .filter(ingredient_groups::Column::Id.is_not_in(incoming_group_ids))
        .exec(txn)
        .await?;

    for group_in in ingredient_group {
        // --- STEP A: UPSERT INGREDIENT GROUP ---
        let current_group_id = match group_in.id {
            Some(id) => {
                let existing = ingredient_groups::Entity::find_by_id(id)
                    .one(txn)
                    .await?
                    .ok_or(Error::NotFound(json!({"error": "Group not found"})))?;

                let mut am: ingredient_groups::ActiveModel = existing.into();
                am.position = Set(group_in.position);
                am.update(txn).await?.id
            }
            None => {
                ingredient_groups::ActiveModel {
                    recipe_id: Set(recipe_id),
                    position: Set(group_in.position),
                    ..Default::default()
                }
                    .insert(txn)
                    .await?
                    .id
            }
        };

        // --- STEP B: RECONCILE GROUP TRANSLATIONS ---
        let incoming_g_trans_ids: Vec<Uuid> = group_in.translations.iter().filter_map(|t| t.id).collect();
        ingredient_group_translations::Entity::delete_many()
            .filter(ingredient_group_translations::Column::IngredientGroupId.eq(current_group_id))
            .filter(ingredient_group_translations::Column::Id.is_not_in(incoming_g_trans_ids))
            .exec(txn)
            .await?;

        for t_in in group_in.translations {
            match t_in.id {
                Some(id) => {
                    let existing = ingredient_group_translations::Entity::find_by_id(id).one(txn).await?.unwrap();
                    let mut am: ingredient_group_translations::ActiveModel = existing.into();
                    am.title = Set(t_in.title);
                    am.language_code = Set(t_in.language_code);
                    am.update(txn).await?;
                }
                None => {
                    ingredient_group_translations::ActiveModel {
                        ingredient_group_id: Set(current_group_id),
                        language_code: Set(t_in.language_code),
                        title: Set(t_in.title),
                        ..Default::default()
                    }.insert(txn).await?;
                }
            }
        }

        // --- STEP C: RECONCILE INGREDIENTS (The Line Items) ---
        let incoming_ing_ids: Vec<Uuid> = group_in.ingredients.iter().filter_map(|i| i.id).collect();

        // Delete ingredients belonging to this group that aren't in the incoming list
        ingredients::Entity::delete_many()
            .filter(ingredients::Column::IngredientGroupId.eq(current_group_id))
            .filter(ingredients::Column::Id.is_not_in(incoming_ing_ids))
            .exec(txn)
            .await?;

        for ing_in in group_in.ingredients {
            let current_ing_id = match ing_in.id {
                Some(id) => {
                    let existing = ingredients::Entity::find_by_id(id).one(txn).await?.unwrap();
                    let mut am: ingredients::ActiveModel = existing.into();
                    am.quantity = Set(ing_in.quantity);
                    am.unit_id = Set(ing_in.unit_id);
                    am.position = Set(ing_in.position);
                    am.update(txn).await?.id
                }
                None => {
                    ingredients::ActiveModel {
                        ingredient_group_id: Set(current_group_id),
                        quantity: Set(ing_in.quantity),
                        unit_id: Set(ing_in.unit_id),
                        position: Set(ing_in.position),
                        ..Default::default()
                    }.insert(txn).await?.id
                }
            };

            // --- STEP D: RECONCILE INGREDIENT TRANSLATIONS (Names & Notes) ---
            // Combine translations (names) and note_translations from the DTO
            // into the single ingredient_translations table.

            // 1. Delete all existing translations for this ingredient and re-insert
            // OR do a fine-grained reconcile. Re-inserting is often cleaner for translations.
            ingredient_translations::Entity::delete_many()
                .filter(ingredient_translations::Column::IngredientId.eq(current_ing_id))
                .exec(txn)
                .await?;

            for t_in in ing_in.translations {
                // Find matching note for this language if it exists in the input

                ingredient_translations::ActiveModel {
                    ingredient_id: Set(current_ing_id),
                    language_code: Set(t_in.language_code),
                    data: Set(t_in.data),
                    note: Set(t_in.note),
                    ..Default::default()
                }.insert(txn).await?;
            }
        }
    }
    Ok(())
}