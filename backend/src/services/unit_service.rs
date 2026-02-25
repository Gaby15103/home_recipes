use crate::dto::unit_dto::{UnitDto, UnitInputDto};
use crate::errors::Error;
use crate::repositories::unit_repository;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<UnitDto>, Error> {
    let result = unit_repository::get_active_units(db).await?;
    Ok(result)
}
pub async fn create(db: &DatabaseConnection, new_unit: UnitInputDto) -> Result<UnitDto, Error> {
    let result = unit_repository::create_unit(db, new_unit).await?;
    Ok(result)
}
pub async fn update(db: &DatabaseConnection, updated_unit: UnitDto) -> Result<UnitDto, Error> {
    let result = unit_repository::update_unit(db, updated_unit.id, updated_unit).await?;
    Ok(result)
}
pub async fn get(db: &DatabaseConnection, unit_i: Uuid) -> Result<UnitDto, Error> {
    let result = unit_repository::find_by_id(db, unit_i).await?;
    Ok(result)
}
