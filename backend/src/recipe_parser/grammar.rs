use super::dictionary::WordType;
use crate::dto::ingredient_dto::{IngredientInput, IngredientTranslationInput};
use crate::dto::ingredient_group_dto::{IngredientGroupInput, IngredientGroupTranslationInput};
use crate::dto::recipe_dto::{CreateRecipeInput, RecipeTranslationInput};
use crate::dto::step_dto::{StepInput, StepTranslationInput};
use crate::dto::step_group_dto::{StepGroupInput, StepGroupTranslationInput};
use crate::dto::unit_dto::UnitDto;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use uuid::Uuid;

pub fn map_to_dto(tokens: Vec<WordType>, known_units: Vec<UnitDto>) -> CreateRecipeInput {
    let mut recipe = CreateRecipeInput {
        primary_language: "en".to_string(),
        translations: vec![RecipeTranslationInput {
            language_code: "en".to_string(),
            title: "Scanned Recipe".to_string(),
            description: "".to_string(),
        }],
        image_url: "".to_string(),
        servings: 1,
        prep_time_minutes: 0,
        cook_time_minutes: 0,
        author_id: None,
        author: None,
        is_private: false,
        tags: vec![],
        ingredient_groups: vec![IngredientGroupInput {
            position: 1,
            translations: vec![IngredientGroupTranslationInput {
                language_code: "en".to_string(),
                title: "Ingredients".to_string(),
            }],
            ingredients: vec![],
        }],
        step_groups: vec![StepGroupInput {
            position: 1,
            translations: vec![StepGroupTranslationInput {
                language_code: "en".to_string(),
                title: "Instructions".to_string(),
            }],
            steps: vec![],
        }],
    };

    let mut current_qty = Decimal::ZERO;
    // Default to a fallback UUID or the first available unit
    let mut current_unit_id = known_units
        .first()
        .map(|u| u.id)
        .unwrap_or_else(Uuid::new_v4);
    let mut is_parsing_ingredients = true;

    for token in tokens {
        match token {
            WordType::Quantity(q) => {
                current_qty = Decimal::from_f32(q).unwrap_or(Decimal::ZERO);
                is_parsing_ingredients = true;
            }

            WordType::Unit(u_name) => {
                // Match against name_en, name_fr, or the symbol (e.g., "g" or "ml")
                if let Some(u) = known_units.iter().find(|u| {
                    u.name_en.to_lowercase() == u_name.to_lowercase()
                        || u.name_fr.to_lowercase() == u_name.to_lowercase()
                        || u.symbol.to_lowercase() == u_name.to_lowercase()
                }) {
                    current_unit_id = u.id;
                }
            }

            WordType::Ingredient(name) => {
                let ing = IngredientInput {
                    quantity: current_qty,
                    unit_id: current_unit_id,
                    position: (recipe.ingredient_groups[0].ingredients.len() + 1) as i32,
                    translations: vec![IngredientTranslationInput {
                        language_code: "en".to_string(),
                        data: name,
                        note: None,
                    }],
                };
                recipe.ingredient_groups[0].ingredients.push(ing);
                current_qty = Decimal::ZERO; // Reset qty after consumption
            }

            WordType::Text(t) => {
                let lower = t.to_lowercase();

                // Switch detection
                if ["steps", "instructions", "préparation", "preparation"].contains(&lower.as_str())
                {
                    is_parsing_ingredients = false;
                    continue;
                }

                if is_parsing_ingredients {
                    // If we haven't set a title, use the first free text as the title
                    if recipe.translations[0].title == "Scanned Recipe" {
                        recipe.translations[0].title = t;
                    }
                    // Otherwise, treat it as a generic ingredient name if no WordType::Ingredient was triggered
                    else {
                        let ing = IngredientInput {
                            quantity: current_qty,
                            unit_id: current_unit_id,
                            position: (recipe.ingredient_groups[0].ingredients.len() + 1) as i32,
                            translations: vec![IngredientTranslationInput {
                                language_code: "en".to_string(),
                                data: t,
                                note: None,
                            }],
                        };
                        recipe.ingredient_groups[0].ingredients.push(ing);
                        current_qty = Decimal::ZERO;
                    }
                } else {
                    // Add to steps
                    let step = StepInput {
                        position: (recipe.step_groups[0].steps.len() + 1) as i32,
                        image_url: None,
                        duration_minutes: None,
                        translations: vec![StepTranslationInput {
                            language_code: "en".to_string(),
                            instruction: t,
                        }],
                    };
                    recipe.step_groups[0].steps.push(step);
                }
            }
        }
    }

    recipe
}

// Helper trait to check if title is still the placeholder
trait TitleCheck {
    fn name_is_default(&self) -> bool;
}
impl TitleCheck for CreateRecipeInput {
    fn name_is_default(&self) -> bool {
        self.translations[0].title == "Scanned Recipe"
    }
}
