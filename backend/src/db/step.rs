use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use actix_multipart::form::tempfile::TempFile;
use crate::dto::{StepGroupInput, StepGroupResponse, StepGroupUpdate, StepImageMeta, StepResponse};
use crate::models::{Step, StepGroup, StepGroupTranslation, StepTranslation};
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
    use crate::schema::{step_groups, steps, step_group_translations, step_translations};

    let image_map: HashMap<(usize, usize), usize> = step_image_meta
        .iter()
        .map(|m| ((m.group_position, m.step_position), m.index))
        .collect();

    let step_dir: PathBuf = PathBuf::from("assets/steps");
    fs::create_dir_all(&step_dir).map_err(|_| diesel::result::Error::RollbackTransaction)?;

    let mut result_groups = Vec::with_capacity(groups.len());

    for group in groups {
        // --- Insert StepGroup
        let group_id: Uuid = diesel::insert_into(step_groups::table)
            .values(step_groups::recipe_id.eq(recipe_id))
            .returning(step_groups::id)
            .get_result(conn)?;

        // --- Insert StepGroupTranslations
        for tr in &group.translations {
            diesel::insert_into(step_group_translations::table)
                .values((
                    step_group_translations::step_group_id.eq(group_id),
                    step_group_translations::language_code.eq(&tr.language),
                    step_group_translations::title.eq(&tr.title),
                ))
                .execute(conn)?;
        }

        let mut step_responses = Vec::new();

        for step in group.steps {
            // --- Insert Step
            let step_id: Uuid = diesel::insert_into(steps::table)
                .values(steps::step_group_id.eq(group_id))
                .returning(steps::id)
                .get_result(conn)?;

            // --- Insert StepTranslations
            for tr in &step.translation {
                diesel::insert_into(step_translations::table)
                    .values((
                        step_translations::step_id.eq(step_id),
                        step_translations::language_code.eq(&tr.language),
                        step_translations::instruction.eq(&tr.instruction),
                    ))
                    .execute(conn)?;
            }

            // --- Attach image if present
            let mut image_url = None;
            if let Some(&image_index) = image_map.get(&(group.position as usize, step.position as usize)) {
                let temp_file = &mut images[image_index];
                let ext = temp_file
                    .file_name
                    .as_deref()
                    .and_then(|n| Path::new(n).extension())
                    .and_then(|e| e.to_str())
                    .unwrap_or("png");

                let file_name = format!("step_{}_{}_{}.{}", step_id, Uuid::new_v4(), chrono::Utc::now().timestamp(), ext);
                let disk_path = step_dir.join(&file_name);

                fs::copy(temp_file.file.path(), &disk_path).map_err(|_| diesel::result::Error::RollbackTransaction)?;
                image_url = Some(format!("/assets/steps/{}", file_name));

                diesel::update(steps::table.find(step_id))
                    .set(steps::image_url.eq(&image_url))
                    .execute(conn)?;
            }

            step_responses.push(StepResponse {
                id: step_id,
                step_group_id: group_id,
                position: step.position,
                duration_minutes: step.duration_minutes,
                image_url,
                translations: step.translation.into_iter().map(|t| crate::dto::StepTranslationResponse {
                    language: t.language,
                    instruction: t.instruction,
                }).collect(),
            });
        }

        // --- Collect StepGroupResponse
        let translations = group.translations.into_iter().map(|t| crate::dto::StepGroupTranslationResponse {
            language: t.language,
            title: t.title,
        }).collect();

        result_groups.push(StepGroupResponse {
            id: group_id,
            position: group.position,
            translations,
            steps: step_responses,
        });
    }

    Ok(result_groups)
}



pub fn fetch_step_groups_for_recipe(
    conn: &mut PgConnection,
    recipe_id: Uuid,
    language_code: Option<&str>,
    fallback_language: &str,
) -> Result<Vec<StepGroupResponse>, diesel::result::Error> {
    use crate::schema::{
        step_groups,
        steps,
        step_group_translations,
        step_translations,
    };

    // --- Fetch all step groups for the recipe
    let groups: Vec<StepGroup> = step_groups::table
        .filter(step_groups::recipe_id.eq(recipe_id))
        .order(step_groups::position.asc())
        .load(conn)?;

    let mut result = Vec::with_capacity(groups.len());

    for group in groups {
        // --- Fetch group translations
        let group_tr_query = step_group_translations::table
            .filter(step_group_translations::step_group_id.eq(group.id))
            .into_boxed();

        let group_tr_query = if let Some(lang) = language_code {
            group_tr_query.filter(
                step_group_translations::language_code
                    .eq(lang)
                    .or(step_group_translations::language_code.eq(fallback_language)),
            )
        } else {
            group_tr_query
        };

        let group_translations: Vec<StepGroupTranslation> = group_tr_query.load(conn)?;

        let group_translations_resp = group_translations
            .into_iter()
            .map(|t| crate::dto::StepGroupTranslationResponse {
                language: t.language_code,
                title: t.title,
            })
            .collect::<Vec<_>>();

        // ---------- STEPS ----------
        let step_rows: Vec<Step> = steps::table
            .filter(steps::step_group_id.eq(group.id))
            .order(steps::position.asc())
            .load(conn)?;

        let mut step_responses = Vec::with_capacity(step_rows.len());

        for step in step_rows {
            // --- Fetch step translations
            let step_tr_query = step_translations::table
                .filter(step_translations::step_id.eq(step.id))
                .into_boxed();

            let step_tr_query = if let Some(lang) = language_code {
                step_tr_query.filter(
                    step_translations::language_code
                        .eq(lang)
                        .or(step_translations::language_code.eq(fallback_language)),
                )
            } else {
                step_tr_query
            };

            let translations: Vec<StepTranslation> = step_tr_query.load(conn)?;

            let translations_resp = translations
                .into_iter()
                .map(|t| crate::dto::StepTranslationResponse {
                    language: t.language_code,
                    instruction: t.instruction,
                })
                .collect::<Vec<_>>();

            step_responses.push(crate::dto::StepResponse {
                id: step.id,
                step_group_id: step.step_group_id,
                position: step.position,
                duration_minutes: step.duration_minutes,
                image_url: step.image_url,
                translations: translations_resp,
            });
        }

        // --- Build StepGroupResponse
        result.push(crate::dto::StepGroupResponse {
            id: group.id,
            position: group.position,
            translations: group_translations_resp,
            steps: step_responses,
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
    fallback_language: &str,
) -> Result<Vec<StepGroupResponse>, diesel::result::Error> {
    use crate::schema::{step_groups, steps, step_group_translations, step_translations};

    let image_map: HashMap<(usize, usize), usize> = step_image_meta
        .iter()
        .map(|m| ((m.group_position, m.step_position), m.index))
        .collect();

    let step_dir: PathBuf = PathBuf::from("assets/steps");
    fs::create_dir_all(&step_dir).map_err(|_| diesel::result::Error::RollbackTransaction)?;

    let existing_group_ids: HashSet<Uuid> = step_groups::table
        .filter(step_groups::recipe_id.eq(recipe_id))
        .select(step_groups::id)
        .load::<Uuid>(conn)?
        .into_iter()
        .collect();

    let mut kept_group_ids = HashSet::new();

    for group in &groups {
        // --- Upsert StepGroup
        let group_id = if let Some(id) = group.id {
            diesel::update(step_groups::table.find(id))
                .set(step_groups::position.eq(group.position))
                .execute(conn)?;
            kept_group_ids.insert(id);
            id
        } else {
            let new_id: Uuid = diesel::insert_into(step_groups::table)
                .values(step_groups::recipe_id.eq(recipe_id))
                .returning(step_groups::id)
                .get_result(conn)?;
            kept_group_ids.insert(new_id);
            new_id
        };

        // --- Upsert StepGroupTranslations
        for tr in &group.translations {
            let exists: Option<Uuid> = step_group_translations::table
                .filter(step_group_translations::step_group_id.eq(group_id))
                .filter(step_group_translations::language_code.eq(&tr.language))
                .select(step_group_translations::id)
                .first(conn)
                .optional()?;

            if let Some(tr_id) = exists {
                diesel::update(step_group_translations::table.find(tr_id))
                    .set(step_group_translations::title.eq(&tr.title))
                    .execute(conn)?;
            } else {
                diesel::insert_into(step_group_translations::table)
                    .values((
                        step_group_translations::step_group_id.eq(group_id),
                        step_group_translations::language_code.eq(&tr.language),
                        step_group_translations::title.eq(&tr.title),
                    ))
                    .execute(conn)?;
            }
        }

        // --- Steps handling
        let existing_step_ids: HashSet<Uuid> = steps::table
            .filter(steps::step_group_id.eq(group_id))
            .select(steps::id)
            .load::<Uuid>(conn)?
            .into_iter()
            .collect();

        let mut kept_step_ids = HashSet::new();

        for step in &group.steps {
            // Upsert Step
            let step_id = if let Some(id) = step.id {
                diesel::update(steps::table.find(id))
                    .set((
                        steps::position.eq(step.position),
                        steps::duration_minutes.eq(step.duration_minutes),
                        steps::image_url.eq(step.image_url.clone()), // optional: update image if present
                    ))
                    .execute(conn)?;
                id
            } else {
                let new_id: Uuid = diesel::insert_into(steps::table)
                    .values(steps::step_group_id.eq(group_id))
                    .returning(steps::id)
                    .get_result(conn)?;
                kept_step_ids.insert(new_id);
                new_id
            };

            // Upsert StepTranslations
            for tr in &step.translations {
                let exists: Option<Uuid> = step_translations::table
                    .filter(step_translations::step_id.eq(step_id))
                    .filter(step_translations::language_code.eq(&tr.language))
                    .select(step_translations::id)
                    .first(conn)
                    .optional()?;

                if let Some(tr_id) = exists {
                    diesel::update(step_translations::table.find(tr_id))
                        .set(step_translations::instruction.eq(&tr.instruction))
                        .execute(conn)?;
                } else {
                    diesel::insert_into(step_translations::table)
                        .values((
                            step_translations::step_id.eq(step_id),
                            step_translations::language_code.eq(&tr.language),
                            step_translations::instruction.eq(&tr.instruction),
                        ))
                        .execute(conn)?;
                }
            }

            // --- Handle image
            if let Some(&image_index) = image_map.get(&(group.position as usize, step.position as usize)) {
                let temp_file = &mut images[image_index];
                let ext = temp_file
                    .file_name
                    .as_deref()
                    .and_then(|n| Path::new(n).extension())
                    .and_then(|e| e.to_str())
                    .unwrap_or("png");

                let file_name = format!("step_{}_{}_{}.{}", step_id, Uuid::new_v4(), chrono::Utc::now().timestamp(), ext);
                let disk_path = step_dir.join(&file_name);

                fs::copy(temp_file.file.path(), &disk_path).map_err(|_| diesel::result::Error::RollbackTransaction)?;
                let image_url = format!("/assets/steps/{}", file_name);

                diesel::update(steps::table.find(step_id))
                    .set(steps::image_url.eq(image_url))
                    .execute(conn)?;
            }
        }

        // --- Delete removed steps
        let removed_steps: Vec<Uuid> = existing_step_ids.difference(&kept_step_ids).cloned().collect();
        if !removed_steps.is_empty() {
            diesel::delete(steps::table.filter(steps::id.eq_any(removed_steps)))
                .execute(conn)?;
        }
    }

    // --- Delete removed groups
    let removed_groups: Vec<Uuid> = existing_group_ids.difference(&kept_group_ids).cloned().collect();
    if !removed_groups.is_empty() {
        diesel::delete(step_groups::table.filter(step_groups::id.eq_any(removed_groups)))
            .execute(conn)?;
    }

    // --- Return updated state
    fetch_step_groups_for_recipe(conn, recipe_id,Some("fr"),fallback_language)
}