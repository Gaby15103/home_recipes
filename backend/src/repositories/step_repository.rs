use crate::dto::step_dto::{StepInput, StepViewDto};
use crate::errors::Error;
use entity::{step_translations, steps};
use sea_orm::{ActiveModelTrait, DatabaseTransaction, Set};
use uuid::Uuid;

pub async fn create(
    txn: &DatabaseTransaction,
    group_id: Uuid,
    input: StepInput,
    lang: &str,
) -> Result<StepViewDto, Error> {
    let step = steps::ActiveModel {
        step_group_id: Set(group_id),
        position: Set(input.position),
        image_url: Set(input.image_url),
        duration_minutes: Set(input.duration_minutes),
        ..Default::default()
    }
    .insert(txn)
    .await?;
    
    let mut display_instruction = String::new();
    for trans in input.translations {
        step_translations::ActiveModel {
            step_id: Set(step.id),
            language_code: Set(trans.language_code.clone()),
            instruction: Set(trans.instruction.clone()),
            ..Default::default()
        }
        .insert(txn)
        .await?;

        if trans.language_code == lang {
            display_instruction = trans.instruction;
        }
    }

    Ok(StepViewDto {
        id: step.id,
        instruction: display_instruction,
        step_group_id: group_id,
        position: step.position,
        image_url: step.image_url,
        duration_minutes: step.duration_minutes,
    })
}
