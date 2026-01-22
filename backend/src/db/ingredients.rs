use diesel::prelude::*;
use uuid::Uuid;
use crate::dto::{IngredientGroupInput, IngredientResponse, IngredientGroupResponse, IngredientGroupUpdate};
use crate::models::{Ingredient, IngredientGroup, RecipeIngredient};
use crate::prelude::*;
use crate::schema::{ingredient_groups, ingredients, recipe_ingredients};
use crate::utils::unit::IngredientUnit;

pub fn create_ingredient_groups(
    conn: &mut PgConnection,
    recipe_id_val: Uuid,
    groups: Vec<IngredientGroupInput>,
) -> Result<Vec<IngredientGroupResponse>, diesel::result::Error> {
    use crate::schema::{
        ingredient_groups::dsl as ig,
        ingredients::dsl as ing,
        recipe_ingredients::dsl as ri,
    };

    let mut result_groups = Vec::with_capacity(groups.len());

    for group in groups {
        // --- create ingredient group
        let group_id: Uuid = diesel::insert_into(ig::ingredient_groups)
            .values((
                ig::recipe_id.eq(recipe_id_val),
                ig::title.eq(&group.title),
                ig::position.eq(group.position),
            ))
            .returning(ig::id)
            .get_result(conn)?;

        let mut ingredients_resp = Vec::with_capacity(group.ingredients.len());

        for ingredient in group.ingredients {
            let normalized_name = ingredient.name.trim().to_lowercase();

            // --- find or create ingredient
            let ingredient_id: Uuid = match ing::ingredients
                .filter(ing::name.eq(&normalized_name))
                .select(ing::id)
                .first::<Uuid>(conn)
            {
                Ok(id) => id,
                Err(diesel::result::Error::NotFound) => {
                    diesel::insert_into(ing::ingredients)
                        .values(ing::name.eq(&normalized_name))
                        .returning(ing::id)
                        .get_result(conn)?
                }
                Err(e) => return Err(e),
            };

            // --- link recipe <-> ingredient
            diesel::insert_into(ri::recipe_ingredients)
                .values((
                    ri::ingredient_group_id.eq(group_id),
                    ri::ingredient_id.eq(ingredient_id),
                    ri::quantity.eq(&ingredient.quantity),
                    ri::unit.eq(ingredient.unit.to_string()),
                    ri::note.eq(&ingredient.note),
                    ri::position.eq(ingredient.position),
                ))
                .execute(conn)?;

            ingredients_resp.push(IngredientResponse {
                id: ingredient_id,
                name: normalized_name,
                quantity: ingredient.quantity,
                unit: ingredient.unit,
                note: ingredient.note,
                position: ingredient.position,
            });
        }

        result_groups.push(IngredientGroupResponse {
            id: group_id,
            title: group.title,
            position: group.position,
            ingredients: ingredients_resp,
        });
    }

    Ok(result_groups)
}

pub fn fetch_ingredient_groups_for_recipe(
    conn: &mut PgConnection,
    recipe_id: Uuid,
) -> Result<Vec<IngredientGroupResponse>, diesel::result::Error> {
    use crate::schema::{ingredient_groups, ingredients, recipe_ingredients};
    use diesel::prelude::*;

    // Fetch all ingredient groups for this recipe
    let groups: Vec<IngredientGroup> = ingredient_groups::table
        .filter(ingredient_groups::recipe_id.eq(recipe_id))
        .order(ingredient_groups::position.asc())
        .load(conn)?;

    let mut result = Vec::with_capacity(groups.len());

    for group in groups {
        // Use LEFT JOIN so it won't fail if no ingredients exist
        let ingredients_rows: Vec<(Ingredient, Option<RecipeIngredient>)> =
            ingredients::table
                .left_join(recipe_ingredients::table.on(
                    recipe_ingredients::ingredient_id.eq(ingredients::id)
                        .and(recipe_ingredients::ingredient_group_id.eq(group.id))
                ))
                .order(recipe_ingredients::position.asc())
                .load(conn)?;

        let ingredients = ingredients_rows
            .into_iter()
            .filter_map(|(ingredient, ri_opt)| {
                ri_opt.map(|ri| IngredientResponse {
                    id: ingredient.id,
                    name: ingredient.name,
                    quantity: ri.quantity,
                    unit: ri.unit.parse().unwrap_or(IngredientUnit::Gram),
                    note: ri.note,
                    position: ri.position,
                })
            })
            .collect::<Vec<_>>();

        result.push(IngredientGroupResponse {
            id: group.id,
            title: group.title,
            position: group.position,
            ingredients,
        });
    }

    Ok(result)
}

pub fn sync_ingredient_groups(
    conn: &mut PgConnection,
    recipe_id: Uuid,
    groups: Vec<IngredientGroupUpdate>,
) -> Result<Vec<IngredientGroupResponse>, diesel::result::Error> {
    use crate::schema::{
        ingredient_groups::dsl as ig,
        ingredients::dsl as ing,
        recipe_ingredients::dsl as ri,
    };
    use std::collections::HashSet;

    // --- Fetch existing group ids
    let existing_ids: HashSet<Uuid> = ig::ingredient_groups
        .filter(ig::recipe_id.eq(recipe_id))
        .select(ig::id)
        .load::<Uuid>(conn)?
        .into_iter()
        .collect();

    let mut kept_ids = HashSet::new();

    for group in &groups {
        let group_id = if let Some(id) = group.id {
            // --- Update group
            diesel::update(ig::ingredient_groups.find(id))
                .set((
                    ig::title.eq(&group.title),
                    ig::position.eq(group.position),
                ))
                .execute(conn)?;

            kept_ids.insert(id);
            id
        } else {
            // --- Insert group
            let new_id: Uuid = diesel::insert_into(ig::ingredient_groups)
                .values((
                    ig::recipe_id.eq(recipe_id),
                    ig::title.eq(&group.title),
                    ig::position.eq(group.position),
                ))
                .returning(ig::id)
                .get_result(conn)?;

            kept_ids.insert(new_id);
            new_id
        };

        // ---------- Ingredients ----------
        let existing_ing_ids: HashSet<Uuid> = ri::recipe_ingredients
            .filter(ri::ingredient_group_id.eq(group_id))
            .select(ri::id)
            .load::<Uuid>(conn)?
            .into_iter()
            .collect();

        let mut kept_ing_ids = HashSet::new();

        for ingredient in &group.ingredients {
            // Normalize name
            let normalized_name = ingredient.name.trim().to_lowercase();

            // Find or create ingredient
            let ingredient_id: Uuid = match ing::ingredients
                .filter(ing::name.eq(&normalized_name))
                .select(ing::id)
                .first::<Uuid>(conn)
            {
                Ok(id) => id,
                Err(diesel::result::Error::NotFound) => {
                    diesel::insert_into(ing::ingredients)
                        .values(ing::name.eq(&normalized_name))
                        .returning(ing::id)
                        .get_result(conn)?
                }
                Err(e) => return Err(e),
            };

            if let Some(ri_id) = ingredient.id {
                // --- Update recipe_ingredient
                diesel::update(ri::recipe_ingredients.find(ri_id))
                    .set((
                        ri::ingredient_id.eq(ingredient_id),
                        ri::quantity.eq(&ingredient.quantity),
                        ri::unit.eq(ingredient.unit.to_string()),
                        ri::note.eq(&ingredient.note),
                        ri::position.eq(ingredient.position),
                    ))
                    .execute(conn)?;

                kept_ing_ids.insert(ri_id);
            } else {
                // --- Insert recipe_ingredient
                let new_ri_id: Uuid = diesel::insert_into(ri::recipe_ingredients)
                    .values((
                        ri::ingredient_group_id.eq(group_id),
                        ri::ingredient_id.eq(ingredient_id),
                        ri::quantity.eq(&ingredient.quantity),
                        ri::unit.eq(ingredient.unit.to_string()),
                        ri::note.eq(&ingredient.note),
                        ri::position.eq(ingredient.position),
                    ))
                    .returning(ri::id)
                    .get_result(conn)?;

                kept_ing_ids.insert(new_ri_id);
            }
        }

        // --- Delete removed ingredients
        let removed_ing_ids: Vec<Uuid> =
            existing_ing_ids.difference(&kept_ing_ids).cloned().collect();

        if !removed_ing_ids.is_empty() {
            diesel::delete(ri::recipe_ingredients.filter(ri::id.eq_any(removed_ing_ids)))
                .execute(conn)?;
        }
    }

    // --- Delete removed groups
    let removed_group_ids: Vec<Uuid> =
        existing_ids.difference(&kept_ids).cloned().collect();

    if !removed_group_ids.is_empty() {
        diesel::delete(ig::ingredient_groups.filter(ig::id.eq_any(removed_group_ids)))
            .execute(conn)?;
    }

    // --- Return updated state
    fetch_ingredient_groups_for_recipe(conn, recipe_id)
}
