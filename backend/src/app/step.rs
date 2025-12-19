use crate::dto::{StepGroupInput, StepGroupResponse, StepResponse};
use crate::models::{Step, StepGroup};
use crate::prelude::*;
use crate::schema::step_groups::dsl::step_groups;
use crate::schema::steps::dsl::steps;
use diesel::prelude::*;
use uuid::Uuid;

pub fn create_step_groups(
    conn: &mut PgConnection,
    recipe_id_val: Uuid,
    groups: Vec<StepGroupInput>,
) -> Result<Vec<StepGroupResponse>, diesel::result::Error> {
    let mut result_groups = Vec::with_capacity(groups.len());

    for group in groups {
        let mut inserted_step_group: StepGroupResponse = StepGroupResponse::from(
            diesel::insert_into(step_groups)
                .values((
                    crate::schema::step_groups::dsl::recipe_id.eq(recipe_id_val),
                    crate::schema::step_groups::dsl::title.eq(&group.title),
                    crate::schema::step_groups::dsl::position.eq(group.position),
                ))
                .returning(StepGroup::as_select())
                .get_result(conn)?,
        );

        for step in &group.steps {
            let inserted_steps: StepResponse = StepResponse::from(
                diesel::insert_into(steps)
                    .values(&(
                        crate::schema::steps::dsl::step_group_id.eq(&inserted_step_group.id),
                        crate::schema::steps::dsl::position.eq(&step.position),
                        crate::schema::steps::dsl::instruction.eq(&step.instruction),
                        crate::schema::steps::dsl::duration_minutes.eq(&step.duration_minutes),
                    ))
                    .returning(Step::as_select())
                    .get_result(conn)?,
            );

            inserted_step_group.steps.push(inserted_steps);
        }
        result_groups.push(inserted_step_group);
    }

    Ok(result_groups)
}
