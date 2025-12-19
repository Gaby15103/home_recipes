use std::fmt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IngredientUnit {
    #[serde(alias = "g", alias = "gram", alias = "grams")]
    Gram,
    #[serde(alias = "kg", alias = "kilogram", alias = "kilograms")]
    Kilogram,

    #[serde(alias = "ml", alias = "milliliter", alias = "milliliters")]
    Milliliter,
    #[serde(alias = "l", alias = "liter", alias = "liters")]
    Liter,

    #[serde(alias = "pc", alias = "piece", alias = "pieces")]
    Piece,

    #[serde(alias = "tsp", alias = "teaspoon", alias = "teaspoons")]
    Teaspoon,
    #[serde(alias = "tbsp", alias = "tablespoon", alias = "tablespoons")]
    Tablespoon,
    #[serde(alias = "cup", alias = "cups", alias = "Cups")]
    Cup,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitCategory {
    Weight,
    Volume,
    Count,
}

impl IngredientUnit {
    pub fn category(&self) -> UnitCategory {
        match self {
            IngredientUnit::Gram | IngredientUnit::Kilogram => UnitCategory::Weight,
            IngredientUnit::Milliliter | IngredientUnit::Liter => UnitCategory::Volume,
            IngredientUnit::Teaspoon
            | IngredientUnit::Tablespoon
            | IngredientUnit::Cup => UnitCategory::Volume,
            IngredientUnit::Piece => UnitCategory::Count,
        }
    }

    pub fn to_base(&self, value: f64) -> f64 {
        match self {
            // Weight → grams
            IngredientUnit::Gram => value,
            IngredientUnit::Kilogram => value * 1000.0,

            // Volume → milliliters
            IngredientUnit::Milliliter => value,
            IngredientUnit::Liter => value * 1000.0,
            IngredientUnit::Teaspoon => value * 5.0,
            IngredientUnit::Tablespoon => value * 15.0,
            IngredientUnit::Cup => value * 240.0,

            // Count → pieces
            IngredientUnit::Piece => value,
        }
    }

    pub fn from_base(&self, value: f64) -> f64 {
        match self {
            IngredientUnit::Gram => value,
            IngredientUnit::Kilogram => value / 1000.0,

            IngredientUnit::Milliliter => value,
            IngredientUnit::Liter => value / 1000.0,
            IngredientUnit::Teaspoon => value / 5.0,
            IngredientUnit::Tablespoon => value / 15.0,
            IngredientUnit::Cup => value / 240.0,

            IngredientUnit::Piece => value,
        }
    }

    pub fn convert(value: f64, from: IngredientUnit, to: IngredientUnit) -> Option<f64> {
        if from.category() != to.category() {
            return None;
        }

        let base = from.to_base(value);
        Some(to.from_base(base))
    }
}

impl fmt::Display for IngredientUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IngredientUnit::Gram => write!(f, "Gram"),
            IngredientUnit::Kilogram => write!(f, "Kilogram"),
            IngredientUnit::Milliliter => write!(f, "Milliliter"),
            IngredientUnit::Liter => write!(f, "Liter"),
            IngredientUnit::Teaspoon => write!(f, "Teaspoon"),
            IngredientUnit::Tablespoon => write!(f, "Tablespoon"),
            IngredientUnit::Cup => write!(f, "Cup"),
            IngredientUnit::Piece => write!(f, "Piece"),
        }
    }
}
impl FromStr for IngredientUnit {

    type Err = ();

    fn from_str(input: &str) -> Result<IngredientUnit, Self::Err> {
        match input {
            "Gram"  => Ok(IngredientUnit::Gram),
            "Kilogram"  => Ok(IngredientUnit::Kilogram),
            "Milliliter"  => Ok(IngredientUnit::Milliliter),
            "Liter"  => Ok(IngredientUnit::Liter),
            "Teaspoon"  => Ok(IngredientUnit::Teaspoon),
            "Tablespoon"  => Ok(IngredientUnit::Tablespoon),
            "Cup"  => Ok(IngredientUnit::Cup),
            "Piece"  => Ok(IngredientUnit::Piece),
            _      => Err(()),
        }
    }
}