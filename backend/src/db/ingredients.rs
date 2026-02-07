use diesel::prelude::*;
use uuid::Uuid;
use crate::dto::{IngredientGroupInput, IngredientResponse, IngredientGroupResponse, IngredientGroupUpdate};
use crate::models::{Ingredient, IngredientGroup, IngredientGroupTranslation, IngredientTranslation, RecipeIngredient, RecipeIngredientTranslation};
use crate::prelude::*;
use crate::schema::{ingredient_groups, ingredients, recipe_ingredients};
use crate::utils::unit::IngredientUnit;

pub fn create_ingredient_groups(
    conn: &mut PgConnection,
    recipe_id_val: Uuid,
    groups: Vec<IngredientGroupInput>,
    fallback_language: &str,
) -> Result<Vec<IngredientGroupResponse>, diesel::result::Error> {

    use crate::schema::{
        ingredient_groups::dsl as ig,
        ingredient_group_translations::dsl as igt,
        ingredients::dsl as ing,
        ingredient_translations::dsl as it,
        recipe_ingredients::dsl as ri,
        recipe_ingredient_translations::dsl as rit,
    };


    for group in groups {

        // ---------- Insert group ----------
        let group_id: Uuid = diesel::insert_into(ig::ingredient_groups)
            .values(ig::recipe_id.eq(recipe_id_val))
            .returning(ig::id)
            .get_result(conn)?;

        // ---------- Insert group translations ----------
        for tr in &group.translations {
            diesel::insert_into(igt::ingredient_group_translations)
                .values((
                    igt::ingredient_group_id.eq(group_id),
                    igt::language_code.eq(&tr.language),
                    igt::title.eq(&tr.title),
                ))
                .execute(conn)?;
        }


        for ingredient in group.ingredients {

            // ---------- Create ingredient ----------
            let ingredient_id: Uuid = diesel::insert_into(ing::ingredients)
                .default_values()
                .returning(ing::id)
                .get_result(conn)?;

            // ---------- Insert ingredient translations ----------
            for tr in &ingredient.translations {
                diesel::insert_into(it::ingredient_translations)
                    .values((
                        it::ingredient_id.eq(ingredient_id),
                        it::language_code.eq(&tr.language),
                        it::name.eq(&tr.name),
                    ))
                    .execute(conn)?;
            }

            // ---------- Insert recipe_ingredient ----------
            let ri_id: Uuid = diesel::insert_into(ri::recipe_ingredients)
                .values((
                    ri::ingredient_group_id.eq(group_id),
                    ri::ingredient_id.eq(ingredient_id),
                    ri::quantity.eq(&ingredient.quantity),
                    ri::unit.eq(ingredient.unit.to_string()),
                    ri::position.eq(ingredient.position),
                ))
                .returning(ri::id)
                .get_result(conn)?;

            // ---------- Insert note translations ----------
            for tr in &ingredient.note_translations {
                diesel::insert_into(rit::recipe_ingredient_translations)
                    .values((
                        rit::recipe_ingredient_id.eq(ri_id),
                        rit::language_code.eq(&tr.language_code),
                        rit::note.eq(&tr.note),
                    ))
                    .execute(conn)?;
            }
        }
    }

    fetch_ingredient_groups_for_recipe(conn, recipe_id_val, Some("fr"),fallback_language)
}


pub fn fetch_ingredient_groups_for_recipe(
    conn: &mut PgConnection,
    recipe_id: Uuid,
    language_code: Option<&str>, // None = all languages, Some("fr") = filter
    fallback_language: &str,
) -> Result<Vec<IngredientGroupResponse>, diesel::result::Error> {
    use crate::schema::{
        ingredient_groups,
        ingredient_group_translations,
        ingredients,
        ingredient_translations,
        recipe_ingredients,
        recipe_ingredient_translations,
    };

    // --- Fetch all ingredient groups
    let groups: Vec<IngredientGroup> = ingredient_groups::table
        .filter(ingredient_groups::recipe_id.eq(recipe_id))
        .order(ingredient_groups::position.asc())
        .load(conn)?;

    let mut result = Vec::with_capacity(groups.len());

    for group in groups {
        // --- Fetch group translations
        let mut group_tr_query = ingredient_group_translations::table
            .filter(ingredient_group_translations::ingredient_group_id.eq(group.id))
            .into_boxed();

        let group_tr_query = if let Some(lang) = language_code {
            group_tr_query.filter(
                ingredient_group_translations::language_code
                    .eq(lang)
                    .or(ingredient_group_translations::language_code.eq(fallback_language))
            )
        } else {
            group_tr_query
        };

        let group_translations = group_tr_query.load::<IngredientGroupTranslation>(conn)?;

        // --- Fetch ingredients with their recipe_ingredients
        let ingredient_rows: Vec<(Ingredient, RecipeIngredient)> = ingredients::table
            .inner_join(recipe_ingredients::table.on(
                recipe_ingredients::ingredient_id.eq(ingredients::id)
                    .and(recipe_ingredients::ingredient_group_id.eq(group.id))
            ))
            .order(recipe_ingredients::position.asc())
            .load(conn)?;

        let mut ingredient_responses = Vec::with_capacity(ingredient_rows.len());

        for (ingredient, ri) in ingredient_rows {
            // --- Fetch ingredient translations
            let mut ing_tr_query = ingredient_translations::table
                .filter(ingredient_translations::ingredient_id.eq(ingredient.id))
                .into_boxed();

            let ing_tr_query = if let Some(lang) = language_code {
                ing_tr_query.filter(
                    ingredient_translations::language_code
                        .eq(lang)
                        .or(ingredient_translations::language_code.eq(fallback_language))
                )
            } else {
                ing_tr_query
            };

            let translations = ing_tr_query.load::<IngredientTranslation>(conn)?;

            // --- Fetch recipe ingredient notes
            let mut note_tr_query = recipe_ingredient_translations::table
                .filter(recipe_ingredient_translations::recipe_ingredient_id.eq(ri.id))
                .into_boxed();

            if let Some(lang) = language_code {
                note_tr_query = note_tr_query.filter(recipe_ingredient_translations::language_code.eq(lang));
            }

            let note_translations = note_tr_query.load::<RecipeIngredientTranslation>(conn)?;

            ingredient_responses.push(
                IngredientResponse::from((ri, ingredient, translations, note_translations))
            );
        }

        result.push(
            IngredientGroupResponse::from((group, group_translations, ingredient_responses))
        );
    }

    Ok(result)
}




pub fn sync_ingredient_groups(
    conn: &mut PgConnection,
    recipe_id: Uuid,
    groups: Vec<IngredientGroupUpdate>,
    fallback_language: &str,
) -> Result<Vec<IngredientGroupResponse>, diesel::result::Error> {

    use crate::schema::{
        ingredient_groups::dsl as ig,
        ingredient_group_translations::dsl as igt,
        ingredients::dsl as ing,
        ingredient_translations::dsl as it,
        recipe_ingredients::dsl as ri,
        recipe_ingredient_translations::dsl as rit,
    };

    use std::collections::HashSet;

    // ---------- Existing groups ----------
    let existing_ids: HashSet<Uuid> = ig::ingredient_groups
        .filter(ig::recipe_id.eq(recipe_id))
        .select(ig::id)
        .load(conn)?
        .into_iter()
        .collect();

    let mut kept_ids = HashSet::new();

    for group in &groups {

        // ---------- UPSERT GROUP ----------
        let group_id = if let Some(id) = group.id {

            diesel::update(ig::ingredient_groups.find(id))
                .set(ig::position.eq(group.position))
                .execute(conn)?;

            id

        } else {

            diesel::insert_into(ig::ingredient_groups)
                .values((
                    ig::recipe_id.eq(recipe_id),
                    ig::position.eq(group.position),
                ))
                .returning(ig::id)
                .get_result(conn)?
        };

        kept_ids.insert(group_id);

        // ---------- UPSERT GROUP TRANSLATIONS ----------
        for tr in &group.translations {

            match igt::ingredient_group_translations
                .filter(igt::ingredient_group_id.eq(group_id))
                .filter(igt::language_code.eq(&tr.language))
                .select(igt::id)
                .first::<Uuid>(conn)
            {
                Ok(id) => {
                    diesel::update(igt::ingredient_group_translations.find(id))
                        .set(igt::title.eq(&tr.title))
                        .execute(conn)?;
                }

                Err(diesel::result::Error::NotFound) => {
                    diesel::insert_into(igt::ingredient_group_translations)
                        .values((
                            igt::ingredient_group_id.eq(group_id),
                            igt::language_code.eq(&tr.language),
                            igt::title.eq(&tr.title),
                        ))
                        .execute(conn)?;
                }

                Err(e) => return Err(e),
            }
        }

        // ---------- EXISTING RECIPE INGREDIENT IDS ----------
        let existing_ing_ids: HashSet<Uuid> = ri::recipe_ingredients
            .filter(ri::ingredient_group_id.eq(group_id))
            .select(ri::id)
            .load(conn)?
            .into_iter()
            .collect();

        let mut kept_ing_ids = HashSet::new();

        for ingredient in &group.ingredients {

            // ---------- UPSERT INGREDIENT ----------
            let ingredient_id = if let Some(existing_ri_id) = ingredient.id {

                ri::recipe_ingredients
                    .find(existing_ri_id)
                    .select(ri::ingredient_id)
                    .first::<Uuid>(conn)?

            } else {

                diesel::insert_into(ing::ingredients)
                    .default_values()
                    .returning(ing::id)
                    .get_result(conn)?
            };

            // ---------- UPSERT INGREDIENT TRANSLATIONS ----------
            for tr in &ingredient.translations {

                match it::ingredient_translations
                    .filter(it::ingredient_id.eq(ingredient_id))
                    .filter(it::language_code.eq(&tr.language))
                    .select(it::id)
                    .first::<Uuid>(conn)
                {
                    Ok(id) => {
                        diesel::update(it::ingredient_translations.find(id))
                            .set(it::name.eq(&tr.name))
                            .execute(conn)?;
                    }

                    Err(diesel::result::Error::NotFound) => {
                        diesel::insert_into(it::ingredient_translations)
                            .values((
                                it::ingredient_id.eq(ingredient_id),
                                it::language_code.eq(&tr.language),
                                it::name.eq(&tr.name),
                            ))
                            .execute(conn)?;
                    }

                    Err(e) => return Err(e),
                }
            }

            // ---------- UPSERT RECIPE INGREDIENT ----------
            let ri_id = if let Some(ri_id) = ingredient.id {

                diesel::update(ri::recipe_ingredients.find(ri_id))
                    .set((
                        ri::ingredient_id.eq(ingredient_id),
                        ri::quantity.eq(&ingredient.quantity),
                        ri::unit.eq(ingredient.unit.to_string()),
                        ri::position.eq(ingredient.position),
                    ))
                    .execute(conn)?;

                ri_id

            } else {

                diesel::insert_into(ri::recipe_ingredients)
                    .values((
                        ri::ingredient_group_id.eq(group_id),
                        ri::ingredient_id.eq(ingredient_id),
                        ri::quantity.eq(&ingredient.quantity),
                        ri::unit.eq(ingredient.unit.to_string()),
                        ri::position.eq(ingredient.position),
                    ))
                    .returning(ri::id)
                    .get_result(conn)?
            };

            kept_ing_ids.insert(ri_id);

            // ---------- UPSERT NOTE TRANSLATIONS ----------
            for tr in &ingredient.note_translations {

                match rit::recipe_ingredient_translations
                    .filter(rit::recipe_ingredient_id.eq(ri_id))
                    .filter(rit::language_code.eq(&tr.language_code))
                    .select(rit::id)
                    .first::<Uuid>(conn)
                {
                    Ok(id) => {
                        diesel::update(rit::recipe_ingredient_translations.find(id))
                            .set(rit::note.eq(&tr.note))
                            .execute(conn)?;
                    }

                    Err(diesel::result::Error::NotFound) => {
                        diesel::insert_into(rit::recipe_ingredient_translations)
                            .values((
                                rit::recipe_ingredient_id.eq(ri_id),
                                rit::language_code.eq(&tr.language_code),
                                rit::note.eq(&tr.note),
                            ))
                            .execute(conn)?;
                    }

                    Err(e) => return Err(e),
                }
            }
        }

        // ---------- Delete removed recipe ingredients ----------
        let removed_ing_ids: Vec<Uuid> =
            existing_ing_ids.difference(&kept_ing_ids).cloned().collect();

        if !removed_ing_ids.is_empty() {
            diesel::delete(
                ri::recipe_ingredients.filter(ri::id.eq_any(removed_ing_ids))
            )
                .execute(conn)?;
        }
    }

    // ---------- Delete removed groups ----------
    let removed_group_ids: Vec<Uuid> =
        existing_ids.difference(&kept_ids).cloned().collect();

    if !removed_group_ids.is_empty() {
        diesel::delete(
            ig::ingredient_groups.filter(ig::id.eq_any(removed_group_ids))
        )
            .execute(conn)?;
    }

    fetch_ingredient_groups_for_recipe(conn, recipe_id, Some("fr"),fallback_language)
}
