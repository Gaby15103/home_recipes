use diesel::prelude::*;
use uuid::Uuid;
use crate::dto::{IngredientGroupInput, IngredientResponse, IngredientGroupResponse};
use crate::prelude::*;

/*
impl Handler<CreateIngredientOuter> for DbExecutor{
    type Result = Result<IngredientResponse>;

    fn handle(&mut self, msg: CreateIngredientOuter, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::ingredients::dsl::*;

        let mut conn = self.0.get()?;

        let new_ingredient =  NewIngredient{
            name: msg.new_ingredient.name,
        };

        let inserted_ingredient: Ingredient = diesel::insert_into(ingredients)
            .values(&new_ingredient)
            .get_result(&mut conn)?;

        Ok(IngredientResponse::from(inserted_ingredient))
    }
}*/

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
/*
impl Handler<UpdateIngredientOuter> for DbExecutor{
    type Result = Result<IngredientResponse>;

    fn handle(&mut self, msg: UpdateIngredientOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::ingredients::dsl::*;

        let mut conn = self.0.get()?;


        let updated_ingredient = IngredientChange {
            name: msg.update_ingredient.name,
        };

        match diesel::update(ingredients.find(msg.update_ingredient.id))
            .set(&updated_ingredient)
            .get_result::<Ingredient>(&mut conn)
        {
            Ok(ingredient) => Ok(ingredient.into()),
            Err(e) => Err(e.into()),
        }
    }
}*/