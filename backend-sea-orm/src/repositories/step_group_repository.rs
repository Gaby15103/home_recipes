use sea_orm::{ActiveModelTrait, DatabaseTransaction, Set};
use uuid::Uuid;
use entity::{step_group_translations, step_groups};
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
        name: display_name,
        recipe_id: group.recipe_id,
        position: group.position,
        steps: step_dtos,
    })
}