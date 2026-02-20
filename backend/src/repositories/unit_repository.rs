use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection,
    DatabaseTransaction, EntityTrait, QueryFilter, QueryOrder, Set
};
use uuid::Uuid;
use entity::ingredient_units::{ActiveModel, Column, Entity as IngredientUnits};

use crate::dto::unit_dto::{UnitDto, UnitInputDto};
use crate::errors::Error;
/// 1. Get all units (Public/App use - only active)
pub async fn get_active_units(db: &DatabaseConnection) -> Result<Vec<UnitDto>, Error> {
    let units = IngredientUnits::find()
        .filter(Column::IsActive.eq(true))
        .order_by_asc(Column::NameEn)
        .all(db)
        .await?;

    Ok(units.into_iter().map(UnitDto::from).collect())
}

/// 2. Get all units (Admin use - includes inactive)
pub async fn get_all_admin(db: &DatabaseConnection) -> Result<Vec<UnitDto>, Error> {
    let units = IngredientUnits::find()
        .order_by_asc(Column::Code)
        .all(db)
        .await?;

    Ok(units.into_iter().map(UnitDto::from).collect())
}

/// 3. Create Unit (Supports Transaction)
pub async fn create_unit(
    txn: &DatabaseConnection,
    input: UnitInputDto
) -> Result<UnitDto, Error> {
    let active_model = ActiveModel {
        code: Set(input.code),
        symbol: Set(input.symbol),
        name_fr: Set(input.name_fr),
        name_en: Set(input.name_en),
        system: Set(input.system),
        conversion_factor: Set(input.conversion_factor),
        is_fraction_allowed: Set(input.is_fraction_allowed),
        is_active: Set(input.is_active),
        ..Default::default()
    };

    let model = active_model.insert(txn).await?;
    Ok(UnitDto::from(model))
}

/// 4. Update Unit (Supports Transaction)
pub async fn update_unit(
    txn: &DatabaseConnection,
    id: Uuid,
    input: UnitDto,
) -> Result<UnitDto, Error> {
    let existing = IngredientUnits::find_by_id(id)
        .one(txn)
        .await?
        .ok_or(Error::NotFound(serde_json::json!({"error": "Unit not found"})))?;

    let mut active_model: ActiveModel = existing.into();
    active_model.code = Set(input.code);
    active_model.symbol = Set(input.symbol);
    active_model.name_fr = Set(input.name_fr);
    active_model.name_en = Set(input.name_en);
    active_model.system = Set(input.system);
    active_model.base_unit_id = Set(input.base_unit_id);
    active_model.conversion_factor = Set(input.conversion_factor);
    active_model.is_fraction_allowed = Set(input.is_fraction_allowed);
    active_model.is_active = Set(input.is_active);

    let model = active_model.update(txn).await?;
    Ok(UnitDto::from(model))
}

/// 5. Delete Unit (Soft or Hard)
/// Hard delete only allowed if not used in recipes (Foreign Key will protect this)
pub async fn delete_unit(db: &DatabaseConnection, id: Uuid) -> Result<(), Error> {
    let result = IngredientUnits::delete_by_id(id).exec(db).await?;

    if result.rows_affected == 0 {
        return Err(Error::NotFound(serde_json::json!({"error": "Unit not found"})));
    }

    Ok(())
}

/// 6. Find Single Unit by ID (Helper)
pub async fn find_by_id<C>(db: &C, id: Uuid) -> Result<UnitDto, Error>
where
    C: ConnectionTrait,
{
    let model = IngredientUnits::find_by_id(id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(serde_json::json!({"error": "Unit not found"})))?;

    Ok(UnitDto::from(model))
}