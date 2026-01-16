use std::path::{Path, PathBuf};
use actix_multipart::form::tempfile::TempFile;
use crate::dto::{StepGroupInput, StepGroupResponse, StepGroupUpdate, StepImageMeta, StepResponse};
use crate::models::{Step, StepGroup};
use crate::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::{step_groups, steps};

pub fn create_step_groups(
    conn: &mut PgConnection,
    recipe_id: Uuid,
    groups: Vec<StepGroupInput>,
    mut images: Vec<TempFile>,
    step_image_meta: Vec<StepImageMeta>,
) -> Result<Vec<StepGroupResponse>, diesel::result::Error> {
    use crate::schema::{step_groups::dsl as stg, steps::dsl as st};
    use std::collections::HashMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use uuid::Uuid;

    let image_map: HashMap<(usize, usize), usize> = step_image_meta
        .iter()
        .map(|m| ((m.group_position, m.step_position), m.index))
        .collect();

    let step_dir: PathBuf = PathBuf::from("assets/steps");
    fs::create_dir_all(&step_dir).map_err(|e| {
        log::error!("Failed to create steps directory: {}", e);
        diesel::result::Error::RollbackTransaction
    })?;

    let mut result_groups = Vec::with_capacity(groups.len());

    for group in groups {
        // Insert step group
        let mut inserted_group: StepGroupResponse = StepGroupResponse::from(
            diesel::insert_into(stg::step_groups)
                .values((
                    stg::recipe_id.eq(recipe_id),
                    stg::title.eq(&group.title),
                    stg::position.eq(group.position),
                ))
                .returning(StepGroup::as_select())
                .get_result(conn)?,
        );

        for step in group.steps {
            // Insert step
            let mut inserted_step: StepResponse = StepResponse::from(
                diesel::insert_into(st::steps)
                    .values((
                        st::step_group_id.eq(inserted_group.id),
                        st::position.eq(step.position),
                        st::instruction.eq(&step.instruction),
                        st::duration_minutes.eq(step.duration_minutes),
                    ))
                    .returning(Step::as_select())
                    .get_result(conn)?,
            );

            // Attach image if this step has one
            if let Some(&image_index) =
                image_map.get(&(group.position as usize, step.position as usize))
            {
                let temp_file = &mut images[image_index];

                let ext = temp_file
                    .file_name
                    .as_deref()
                    .and_then(|n| Path::new(n).extension())
                    .and_then(|e| e.to_str())
                    .unwrap_or("png");

                let file_name = format!(
                    "step_{}_{}_{}.{}",
                    inserted_step.id,
                    Uuid::new_v4(),
                    chrono::Utc::now().timestamp(),
                    ext
                );

                let disk_path = step_dir.join(&file_name);

                fs::copy(temp_file.file.path(), &disk_path).map_err(|e| {
                    log::error!("Failed to copy step image: {}", e);
                    diesel::result::Error::RollbackTransaction
                })?;

                let image_url = format!("/assets/steps/{}", file_name);

                diesel::update(st::steps.find(inserted_step.id))
                    .set(st::image_url.eq(&image_url))
                    .execute(conn)?;

                inserted_step.image_url = Some(image_url);
            }

            inserted_group.steps.push(inserted_step);
        }

        result_groups.push(inserted_group);
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


pub fn sync_step_groups(
    conn: &mut PgConnection,
    recipe_id: Uuid,
    groups: Vec<StepGroupUpdate>,
    mut images: Vec<TempFile>,
    step_image_meta: Vec<StepImageMeta>,
) -> Result<Vec<StepGroupResponse>, diesel::result::Error> {
    use crate::schema::{step_groups::dsl as sg, steps::dsl as st};
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use std::path::{Path, PathBuf};

    // Map image position → file index
    let image_map: HashMap<(usize, usize), usize> = step_image_meta
        .iter()
        .map(|m| ((m.group_position, m.step_position), m.index))
        .collect();

    let step_dir: PathBuf = PathBuf::from("assets/steps");
    fs::create_dir_all(&step_dir).map_err(|e| {
        log::error!("Failed to create steps directory: {}", e);
        diesel::result::Error::RollbackTransaction
    })?;

    // --- Fetch existing groups
    let existing_group_ids: HashSet<Uuid> = sg::step_groups
        .filter(sg::recipe_id.eq(recipe_id))
        .select(sg::id)
        .load::<Uuid>(conn)?
        .into_iter()
        .collect();

    let mut kept_group_ids = HashSet::new();

    for group in &groups {
        let group_id = if let Some(id) = group.id {
            diesel::update(sg::step_groups.find(id))
                .set((
                    sg::title.eq(&group.title),
                    sg::position.eq(group.position),
                ))
                .execute(conn)?;
            kept_group_ids.insert(id);
            id
        } else {
            let new_id: Uuid = diesel::insert_into(sg::step_groups)
                .values((
                    sg::recipe_id.eq(recipe_id),
                    sg::title.eq(&group.title),
                    sg::position.eq(group.position),
                ))
                .returning(sg::id)
                .get_result(conn)?;
            kept_group_ids.insert(new_id);
            new_id
        };

        // ---------- Steps ----------
        let existing_step_ids: HashSet<Uuid> = st::steps
            .filter(st::step_group_id.eq(group_id))
            .select(st::id)
            .load::<Uuid>(conn)?
            .into_iter()
            .collect();

        let mut kept_step_ids = HashSet::new();

        for step in &group.steps {
            let step_id = if let Some(id) = step.id {
                diesel::update(st::steps.find(id))
                    .set((
                        st::position.eq(step.position),
                        st::instruction.eq(&step.instruction),
                        st::duration_minutes.eq(step.duration_minutes),
                    ))
                    .execute(conn)?;
                kept_step_ids.insert(id);
                id
            } else {
                let new_id: Uuid = diesel::insert_into(st::steps)
                    .values((
                        st::step_group_id.eq(group_id),
                        st::position.eq(step.position),
                        st::instruction.eq(&step.instruction),
                        st::duration_minutes.eq(step.duration_minutes),
                    ))
                    .returning(st::id)
                    .get_result(conn)?;
                kept_step_ids.insert(new_id);
                new_id
            };

            // ---------- Image handling ----------
            if let Some(&image_index) =
                image_map.get(&(group.position as usize, step.position as usize))
            {
                let temp_file = &mut images[image_index];

                let ext = temp_file
                    .file_name
                    .as_deref()
                    .and_then(|n| Path::new(n).extension())
                    .and_then(|e| e.to_str())
                    .unwrap_or("png");

                let file_name = format!(
                    "step_{}_{}_{}.{}",
                    step_id,
                    Uuid::new_v4(),
                    chrono::Utc::now().timestamp(),
                    ext
                );

                let disk_path = step_dir.join(&file_name);
                fs::copy(temp_file.file.path(), &disk_path).map_err(|e| {
                    log::error!("Failed to copy step image: {}", e);
                    diesel::result::Error::RollbackTransaction
                })?;


                let image_url = format!("/assets/steps/{}", file_name);

                diesel::update(st::steps.find(step_id))
                    .set(st::image_url.eq(image_url))
                    .execute(conn)?;
            }
        }

        // --- Delete removed steps
        let removed_steps: Vec<Uuid> =
            existing_step_ids.difference(&kept_step_ids).cloned().collect();

        if !removed_steps.is_empty() {
            diesel::delete(st::steps.filter(st::id.eq_any(removed_steps)))
                .execute(conn)?;
        }
    }

    // --- Delete removed groups
    let removed_groups: Vec<Uuid> =
        existing_group_ids.difference(&kept_group_ids).cloned().collect();

    if !removed_groups.is_empty() {
        diesel::delete(sg::step_groups.filter(sg::id.eq_any(removed_groups)))
            .execute(conn)?;
    }

    fetch_step_groups_for_recipe(conn, recipe_id)
}
