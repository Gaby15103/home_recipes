use diesel::associations::HasTable;
use crate::dto::{StepGroupInput, StepGroupResponse, StepResponse};
use crate::models::{Step, StepGroup};
use crate::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::{step_groups, steps};

pub fn create_step_groups(
    conn: &mut PgConnection,
    recipe_id_val: Uuid,
    groups: Vec<StepGroupInput>,
) -> Result<Vec<StepGroupResponse>, diesel::result::Error> {
    use crate::schema::{
        step_groups::dsl as stg,
        steps::dsl as st
    };
    let mut result_groups = Vec::with_capacity(groups.len());

    for group in groups {
        let mut inserted_step_group: StepGroupResponse = StepGroupResponse::from(
            diesel::insert_into(stg::step_groups)
                .values((
                    step_groups::dsl::recipe_id.eq(recipe_id_val),
                    step_groups::dsl::title.eq(&group.title),
                    step_groups::dsl::position.eq(group.position),
                ))
                .returning(StepGroup::as_select())
                .get_result(conn)?,
        );

        for step in &group.steps {
            let inserted_steps: StepResponse = StepResponse::from(
                diesel::insert_into(st::steps)
                    .values(&(
                        steps::dsl::step_group_id.eq(&inserted_step_group.id),
                        steps::dsl::position.eq(&step.position),
                        steps::dsl::instruction.eq(&step.instruction),
                        steps::dsl::duration_minutes.eq(&step.duration_minutes),
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

pub fn fetch_step_groups_for_recipe(
    conn: &mut PgConnection,
    recipe_id: Uuid,
) -> Result<Vec<StepGroupResponse>, diesel::result::Error> {

    let groups: Vec<StepGroup> = step_groups::table
        .filter(step_groups::recipe_id.eq(recipe_id))
        .order(step_groups::position.asc())
        .load(conn)?;

    let mut result = Vec::with_capacity(groups.len());

    for group in groups {
        let steps_list: Vec<Step> = steps::table
            .filter(steps::step_group_id.eq(group.id))
            .order(steps::position.asc())
            .load(conn)?;

        let steps = steps_list.into_iter().map(StepResponse::from).collect();

        result.push(StepGroupResponse {
            id: group.id,
            title: group.title,
            position: group.position,
            steps,
        });
    }

    Ok(result)
}
