use std::collections::HashMap;
use sea_orm::{QueryFilter, QueryOrder, RelationTrait};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, QuerySelect, Set};
use serde_json::json;
use uuid::Uuid;
use entity::{recipe_tags, recipe_translations, recipes, step_group_translations, step_groups, step_translations, steps, tags};
use migration::JoinType;
use crate::dto::step_dto::{StepEditorDto, StepTranslationsDto, StepViewDto};
use crate::dto::step_group_dto::{EditStepGroupInput, StepGroupEditorDto, StepGroupInput, StepGroupTranslationDto, StepGroupViewDto};
use crate::errors::Error;
use crate::repositories::step_repository;

pub async fn create_multiple(
    txn: &DatabaseTransaction,
    recipe_id: Uuid,
    inputs: Vec<StepGroupInput>,
    lang: &str,
) -> Result<Vec<StepGroupViewDto>, Error> {
    let mut groups = Vec::new();
    for input in inputs {
        groups.push(create(txn, recipe_id, input, lang).await?);
    }
    Ok(groups)
}

pub async fn create(
    txn: &DatabaseTransaction,
    recipe_id: Uuid,
    input: StepGroupInput,
    lang: &str,
) -> Result<StepGroupViewDto, Error> {
    // 1. Insert Group
    let group = step_groups::ActiveModel {
        recipe_id: Set(recipe_id),
        position: Set(input.position),
        ..Default::default()
    }
        .insert(txn)
        .await?;

    // 2. Handle Group Translations
    let mut display_name = String::new();
    for trans in input.translations {
        step_group_translations::ActiveModel {
            step_group_id: Set(group.id),
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

    // 3. Handle Individual Steps
    let mut step_dtos = Vec::new();
    for step_input in input.steps {
        let step_dto = step_repository::create(
            txn,
            group.id,
            step_input,
            lang
        ).await?;
        step_dtos.push(step_dto);
    }

    Ok(StepGroupViewDto {
        id: group.id,
        title: display_name,
        recipe_id: group.recipe_id,
        position: group.position,
        steps: step_dtos,
    })
}
pub async fn find_by_recipe(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    lang: &str,
    default_lang_code: &str,
) -> Result<Vec<StepGroupViewDto>, Error> {

    let groups_with_trans = step_groups::Entity::find()
        .filter(step_groups::Column::RecipeId.eq(recipe_id))
        .order_by_asc(step_groups::Column::Position)
        .find_with_related(step_group_translations::Entity)
        .all(db)
        .await?;

    // 2. Fetch all Steps and their Translations for these groups
    // We fetch them all at once to avoid a loop
    let group_ids: Vec<Uuid> = groups_with_trans.iter().map(|(g, _)| g.id).collect();

    let steps_with_trans = steps::Entity::find()
        .filter(steps::Column::StepGroupId.is_in(group_ids))
        .order_by_asc(steps::Column::Position)
        .find_with_related(step_translations::Entity)
        .all(db)
        .await?;

    // 3. Map steps into a temporary HashMap grouped by their parent ID
    let mut steps_map: HashMap<Uuid, Vec<StepViewDto>> = HashMap::new();
    for (step, translations) in steps_with_trans {
        let instruction = translations.iter()
            .find(|t| t.language_code == lang)
            .or_else(|| translations.iter().find(|t| t.language_code == *default_lang_code))
            .map(|t| t.instruction.clone())
            .unwrap_or_default();

        steps_map.entry(step.step_group_id).or_default().push(StepViewDto {
            id: step.id,
            instruction,
            step_group_id: step.step_group_id,
            position: step.position,
            image_url: step.image_url,
            duration_minutes: step.duration_minutes,
        });
    }

    // 4. Assemble the final View DTOs
    let mut results = Vec::new();
    for (group, translations) in groups_with_trans {
        let title = translations.iter()
            .find(|t| t.language_code == lang)
            .or_else(|| translations.iter().find(|t| t.language_code == *default_lang_code))
            .map(|t| t.title.clone())
            .unwrap_or_default();

        let group_id = group.id;
        results.push(StepGroupViewDto {
            id: group_id,
            title,
            recipe_id: group.recipe_id,
            position: group.position,
            steps: steps_map.remove(&group_id).unwrap_or_default(),
        });
    }

    Ok(results)
}
pub async fn find_all_by_recipe(
    db: &DatabaseConnection,
    recipe_id: Uuid,
) -> Result<Vec<StepGroupEditorDto>, Error> {
    // 1. Fetch Step Groups + Translations
    let groups_with_trans = step_groups::Entity::find()
        .filter(step_groups::Column::RecipeId.eq(recipe_id))
        .order_by_asc(step_groups::Column::Position)
        .find_with_related(step_group_translations::Entity)
        .all(db)
        .await?;

    let group_ids: Vec<Uuid> = groups_with_trans.iter().map(|(g, _)| g.id).collect();

    // 2. Fetch Steps + Translations
    let steps_with_trans = steps::Entity::find()
        .filter(steps::Column::StepGroupId.is_in(group_ids))
        .order_by_asc(steps::Column::Position)
        .find_with_related(step_translations::Entity)
        .all(db)
        .await?;

    // 3. Group Steps by Group ID using your StepEditorDto
    let mut steps_map: HashMap<Uuid, Vec<StepEditorDto>> = HashMap::new();
    for (step, translations) in steps_with_trans {
        let step_id = step.id;
        steps_map.entry(step.step_group_id).or_default().push(StepEditorDto {
            id: step_id,
            step_group_id: step.step_group_id,
            position: step.position,
            // Map the model Vec to your DTO Vec
            translations: translations.into_iter().map(|t| StepTranslationsDto {
                id: t.id,
                language_code: t.language_code,
                instruction: t.instruction,
            }).collect(),
            image_url: step.image_url,
        });
    }

    // 4. Assemble the StepGroupEditorDto (Make sure you have this DTO defined)
    let results = groups_with_trans.into_iter().map(|(group, translations)| {
        let group_id = group.id;
        StepGroupEditorDto {
            id: group_id,
            recipe_id,
            position: group.position,
            translations: translations.into_iter().map(|t| StepGroupTranslationDto {
                id: t.id,
                language_code: t.language_code,
                title: t.title,
            }).collect(),
            steps: steps_map.remove(&group_id).unwrap_or_default(),
        }
    }).collect();

    Ok(results)
}
pub async fn update(
    txn: &DatabaseTransaction,
    recipe_id: Uuid,
    step_groups: Vec<EditStepGroupInput>,
) -> Result<(), Error> {
    // 1. Delete Step Groups not in incoming list
    let incoming_group_ids: Vec<Uuid> = step_groups.iter().filter_map(|g| g.id).collect();

    step_groups::Entity::delete_many()
        .filter(step_groups::Column::RecipeId.eq(recipe_id))
        .filter(step_groups::Column::Id.is_not_in(incoming_group_ids))
        .exec(txn).await?;

    for group_in in step_groups {
        // --- STEP A: UPSERT STEP GROUP ---
        let current_group_id = match group_in.id {
            Some(id) => {
                let existing = step_groups::Entity::find_by_id(id).one(txn).await?
                    .ok_or(Error::NotFound(json!({"error": "Step group not found"})))?;

                // VERIFICATION: Only update if position changed
                if existing.position != group_in.position {
                    let mut am: step_groups::ActiveModel = existing.into();
                    am.position = Set(group_in.position);
                    am.update(txn).await?.id
                } else {
                    existing.id
                }
            }
            None => {
                step_groups::ActiveModel {
                    recipe_id: Set(recipe_id),
                    position: Set(group_in.position),
                    ..Default::default()
                }.insert(txn).await?.id
            }
        };

        // --- STEP B: RECONCILE GROUP TRANSLATIONS ---
        let incoming_trans_ids: Vec<Uuid> = group_in.translations.iter().filter_map(|t| t.id).collect();
        step_group_translations::Entity::delete_many()
            .filter(step_group_translations::Column::StepGroupId.eq(current_group_id))
            .filter(step_group_translations::Column::Id.is_not_in(incoming_trans_ids))
            .exec(txn).await?;

        for t_in in group_in.translations {
            match t_in.id {
                Some(id) => {
                    let existing = step_group_translations::Entity::find_by_id(id).one(txn).await?.unwrap();
                    // VERIFICATION
                    if existing.title != t_in.title || existing.language_code != t_in.language_code {
                        let mut am: step_group_translations::ActiveModel = existing.into();
                        am.title = Set(t_in.title);
                        am.language_code = Set(t_in.language_code);
                        am.update(txn).await?;
                    }
                }
                None => {
                    step_group_translations::ActiveModel {
                        step_group_id: Set(current_group_id),
                        language_code: Set(t_in.language_code),
                        title: Set(t_in.title),
                        ..Default::default()
                    }.insert(txn).await?;
                }
            }
        }

        // --- STEP C: RECONCILE STEPS ---
        let incoming_step_ids: Vec<Uuid> = group_in.steps.iter().filter_map(|s| s.id).collect();
        steps::Entity::delete_many()
            .filter(steps::Column::StepGroupId.eq(current_group_id))
            .filter(steps::Column::Id.is_not_in(incoming_step_ids))
            .exec(txn).await?;

        for step_in in group_in.steps {
            // VERIFICATION & ID CAPTURE
            let current_step_id = match step_in.id {
                Some(id) => {
                    let existing = steps::Entity::find_by_id(id).one(txn).await?.unwrap();
                    if existing.position != step_in.position || existing.image_url != step_in.image_url {
                        let mut am: steps::ActiveModel = existing.into();
                        am.position = Set(step_in.position);
                        am.image_url = Set(step_in.image_url);
                        am.update(txn).await?.id
                    } else {
                        existing.id
                    }
                }
                None => {
                    steps::ActiveModel {
                        step_group_id: Set(current_group_id),
                        position: Set(step_in.position),
                        image_url: Set(step_in.image_url),
                        ..Default::default()
                    }.insert(txn).await?.id
                }
            };

            // --- STEP D: RECONCILE STEP TRANSLATIONS (Instructions) ---
            let incoming_instruction_ids: Vec<Uuid> = step_in.translations.iter().filter_map(|t| t.id).collect();
            step_translations::Entity::delete_many()
                .filter(step_translations::Column::StepId.eq(current_step_id))
                .filter(step_translations::Column::Id.is_not_in(incoming_instruction_ids))
                .exec(txn).await?;

            for t_in in step_in.translations {
                match t_in.id {
                    Some(id) => {
                        let existing = step_translations::Entity::find_by_id(id).one(txn).await?.unwrap();
                        // VERIFICATION
                        if existing.instruction != t_in.instruction || existing.language_code != t_in.language_code {
                            let mut am: step_translations::ActiveModel = existing.into();
                            am.instruction = Set(t_in.instruction);
                            am.language_code = Set(t_in.language_code);
                            am.update(txn).await?;
                        }
                    }
                    None => {
                        step_translations::ActiveModel {
                            step_id: Set(current_step_id),
                            language_code: Set(t_in.language_code),
                            instruction: Set(t_in.instruction),
                            ..Default::default()
                        }.insert(txn).await?;
                    }
                }
            }
        }
    }
    Ok(())
}