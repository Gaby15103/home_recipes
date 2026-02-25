use entity::ingredient_units;
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema,Clone)]
pub struct UnitDto {
    pub id: Uuid,
    pub code: String,
    pub symbol: String,
    pub name_fr: String,
    pub name_en: String,
    pub system: String,
    pub base_unit_id: Option<Uuid>,
    pub conversion_factor: f64,
    pub is_fraction_allowed: bool,
    pub is_active: bool,
}
#[derive(Debug, Serialize, Deserialize, ToSchema,Clone)]
pub struct UnitInputDto {
    pub code: String,
    pub symbol: String,
    pub name_fr: String,
    pub name_en: String,
    pub system: String,
    pub conversion_factor: f64,
    pub is_fraction_allowed: bool,
    pub is_active: bool,
}
impl From<ingredient_units::Model> for UnitDto {
    fn from(value: ingredient_units::Model) -> Self {
        Self {
            id: value.id,
            code: value.code,
            symbol: value.symbol,
            name_fr: value.name_fr,
            name_en: value.name_en,
            system: value.system,
            base_unit_id: value.base_unit_id,
            conversion_factor: value.conversion_factor,
            is_fraction_allowed: value.is_fraction_allowed,
            is_active: value.is_active,
        }
    }
}