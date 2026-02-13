use std::collections::HashMap;
use sea_orm::{QueryFilter, QueryOrder, RelationTrait};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, QuerySelect, Set};
use uuid::Uuid;
use entity::{recipe_tags, recipes, step_group_translations, step_groups, step_translations, steps, tags};
use migration::JoinType;
use crate::dto::step_dto::StepViewDto;
use crate::dto::step_group_dto::{StepGroupInput, StepGroupViewDto};
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