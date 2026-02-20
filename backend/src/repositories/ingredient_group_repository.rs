use sea_orm::{DatabaseConnection, QueryFilter, QueryOrder};
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, Set};
use serde_json::json;
use uuid::Uuid;
use entity::{ingredient_group_translations, ingredient_groups, ingredient_translations, ingredients, recipe_ingredient_translations, recipe_ingredients};
use crate::dto::ingredient_dto::{IngredientEditorDto, IngredientNoteTranslationsDto, IngredientRecipeViewDto, IngredientTranslationsDto};
use crate::dto::ingredient_group_dto::{EditIngredientGroupInput, IngredientGroupEditorDto, IngredientGroupInput, IngredientGroupTranslationDto, IngredientGroupViewDto};
use crate::dto::recipe_dto::{EditRecipeInput, RecipeViewDto};
use crate::errors::Error;
use crate::repositories::{ingredient_repository, unit_repository};
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
    let mut ingredients_map: HashMap<Uuid, Vec<IngredientRecipeViewDto>> = HashMap::new();

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

        let unit = unit_repository::find_by_id::<DatabaseConnection>(db,rel.unit_id).await?;

        ingredients_map.entry(rel.ingredient_group_id).or_default().push(IngredientRecipeViewDto {
            id: rel.id,
            ingredient_id: rel.ingredient_id,
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
pub async fn find_all_by_recipe(
    db: &DatabaseConnection,
    recipe_id: Uuid,
) -> Result<Vec<IngredientGroupEditorDto>, Error> {
    // 1. Fetch Groups + All Translations
    let groups_with_trans = ingredient_groups::Entity::find()
        .filter(ingredient_groups::Column::RecipeId.eq(recipe_id))
        .order_by_asc(ingredient_groups::Column::Position)
        .find_with_related(ingredient_group_translations::Entity)
        .all(db)
        .await?;

    let group_ids: Vec<Uuid> = groups_with_trans.iter().map(|(g, _)| g.id).collect();
    if group_ids.is_empty() { return Ok(vec![]); }

    // 2. Fetch Recipe-Ingredient Relations + All Note Translations
    let rels_with_notes = recipe_ingredients::Entity::find()
        .filter(recipe_ingredients::Column::IngredientGroupId.is_in(group_ids))
        .order_by_asc(recipe_ingredients::Column::Position)
        .find_with_related(recipe_ingredient_translations::Entity)
        .all(db)
        .await?;

    let ingredient_ids: Vec<Uuid> = rels_with_notes.iter().map(|(rel, _)| rel.ingredient_id).collect();

    let master_translations = ingredient_translations::Entity::find()
        .filter(ingredient_translations::Column::IngredientId.is_in(ingredient_ids))
        .all(db)
        .await?;

    // 3. Map Ingredients into Groups
    let mut ingredients_map: HashMap<Uuid, Vec<IngredientEditorDto>> = HashMap::new();

    for (rel, notes) in rels_with_notes {
        let current_ingredient_id = rel.ingredient_id;

        // Filter master names for this specific ingredient
        let ingredient_names: Vec<IngredientTranslationsDto> = master_translations.iter()
            .filter(|t| t.ingredient_id == current_ingredient_id)
            .map(|t| IngredientTranslationsDto {
                id: t.id.clone(),
                language_code: t.language_code.clone(),
                name: t.name.clone(),
            })
            .collect();

        let unit = unit_repository::find_by_id::<DatabaseConnection>(db,rel.unit_id).await?;

        ingredients_map.entry(rel.ingredient_group_id).or_default().push(IngredientEditorDto {
            id: rel.id,
            ingredient_id: current_ingredient_id,
            quantity: rel.quantity,
            translations: ingredient_names,
            unit,
            position: rel.position,
            note_translation: notes.into_iter().map(|n| IngredientNoteTranslationsDto {
                id: n.id,
                language_code: n.language_code,
                note: n.note.unwrap_or_default(),
            }).collect(),
            unit_id: rel.unit_id,
        });
    }

    // 4. Assemble Final Editor DTOs
    let results = groups_with_trans.into_iter().map(|(group, trans)| {
        let gid = group.id;
        IngredientGroupEditorDto {
            id: gid,
            recipe_id: group.recipe_id,
            position: group.position,
            translations: trans.into_iter().map(|t| IngredientGroupTranslationDto {
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
    // 1. Delete Groups not in incoming list
    let incoming_group_ids: Vec<Uuid> = ingredient_group.iter().filter_map(|g| g.id).collect();

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

                if existing.position != group_in.position {
                    let mut am: ingredient_groups::ActiveModel = existing.into();
                    am.position = Set(group_in.position);
                    am.update(txn).await?.id
                } else {
                    existing.id
                }
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
        let incoming_trans_ids: Vec<Uuid> = group_in.translations.iter().filter_map(|t| t.id).collect();
        ingredient_group_translations::Entity::delete_many()
            .filter(ingredient_group_translations::Column::IngredientGroupId.eq(current_group_id))
            .filter(ingredient_group_translations::Column::Id.is_not_in(incoming_trans_ids))
            .exec(txn)
            .await?;

        for t_in in group_in.translations {
            match t_in.id {
                Some(id) => {
                    let existing = ingredient_group_translations::Entity::find_by_id(id)
                        .one(txn)
                        .await?
                        .ok_or(Error::NotFound(json!({"error": "Group translation not found"})))?;
                    if existing.title != t_in.title || existing.language_code != t_in.language_code {
                        let mut am: ingredient_group_translations::ActiveModel = existing.into();
                        am.title = Set(t_in.title);
                        am.language_code = Set(t_in.language_code);
                        am.update(txn).await?;
                    }
                }
                None => {
                    ingredient_group_translations::ActiveModel {
                        ingredient_group_id: Set(current_group_id),
                        language_code: Set(t_in.language_code),
                        title: Set(t_in.title),
                        ..Default::default()
                    }
                        .insert(txn)
                        .await?;
                }
            }
        }

        // --- STEP C: RECONCILE INGREDIENT LINKS ---
        let incoming_rel_ids: Vec<Uuid> = group_in.ingredients.iter().filter_map(|i| i.id).collect();
        recipe_ingredients::Entity::delete_many()
            .filter(recipe_ingredients::Column::IngredientGroupId.eq(current_group_id))
            .filter(recipe_ingredients::Column::Id.is_not_in(incoming_rel_ids))
            .exec(txn)
            .await?;

        for ing_in in group_in.ingredients {
            // We capture current_rel_id to use in Step D (notes)
            let current_rel_id = match ing_in.id {
                Some(existing_rel_id) => {
                    let rel = recipe_ingredients::Entity::find_by_id(existing_rel_id)
                        .one(txn)
                        .await?
                        .ok_or(Error::NotFound(json!({"error": "Ingredient link not found"})))?;

                    if rel.quantity != ing_in.quantity
                        || rel.unit_id != ing_in.unit_id
                        || rel.position != ing_in.position
                    {
                        let mut am: recipe_ingredients::ActiveModel = rel.into();
                        am.quantity = Set(ing_in.quantity);
                        am.unit_id = Set(ing_in.unit_id);
                        am.position = Set(ing_in.position);
                        am.update(txn).await?.id
                    } else {
                        rel.id
                    }
                }
                None => {
                    // Logic for a new ingredient link:
                    // Note: You must ensure you handle the creation of the master ingredient
                    // name/entry elsewhere if it doesn't exist yet!
                    recipe_ingredients::ActiveModel {
                        ingredient_id: Set(Uuid::new_v4()), // Or find existing master ID
                        ingredient_group_id: Set(current_group_id),
                        quantity: Set(ing_in.quantity),
                        unit_id: Set(ing_in.unit_id),
                        position: Set(ing_in.position),
                        ..Default::default()
                    }
                        .insert(txn)
                        .await?
                        .id
                }
            };

            // --- STEP D: RECONCILE NOTE TRANSLATIONS ---
            if let Some(notes) = ing_in.note {
                let incoming_note_ids: Vec<Uuid> = notes.iter().filter_map(|n| n.id).collect();

                recipe_ingredient_translations::Entity::delete_many()
                    .filter(recipe_ingredient_translations::Column::RecipeIngredientId.eq(current_rel_id))
                    .filter(recipe_ingredient_translations::Column::Id.is_not_in(incoming_note_ids))
                    .exec(txn)
                    .await?;

                for n_in in notes {
                    match n_in.id {
                        Some(id) => {
                            let existing_note = recipe_ingredient_translations::Entity::find_by_id(id)
                                .one(txn)
                                .await?
                                .ok_or(Error::NotFound(json!({"error": "Note not found"})))?;

                            if existing_note.note != Some(n_in.note.clone()) || existing_note.language_code != n_in.language_code {
                                let mut am: recipe_ingredient_translations::ActiveModel = existing_note.into();
                                am.note = Set(Some(n_in.note));
                                am.language_code = Set(n_in.language_code);
                                am.update(txn).await?;
                            }
                        }
                        None => {
                            recipe_ingredient_translations::ActiveModel {
                                recipe_ingredient_id: Set(current_rel_id), // Correctly use the ID from Step C
                                language_code: Set(n_in.language_code),
                                note: Set(Some(n_in.note)),
                                ..Default::default()
                            }
                                .insert(txn)
                                .await?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}