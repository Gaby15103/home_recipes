use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IngredientUnits::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).uuid().primary_key().extra("DEFAULT gen_random_uuid()"))
                    .col(ColumnDef::new(IngredientUnits::Code).string().unique_key().not_null())
                    .col(ColumnDef::new(IngredientUnits::Symbol).string().not_null())
                    .col(ColumnDef::new(IngredientUnits::NameFr).string().not_null())
                    .col(ColumnDef::new(IngredientUnits::NameEn).string().not_null())
                    .col(ColumnDef::new(IngredientUnits::System).string().not_null())
                    .col(ColumnDef::new(IngredientUnits::BaseUnitId).uuid())
                    .col(ColumnDef::new(IngredientUnits::ConversionFactor).double().not_null().default(1.0))
                    .col(ColumnDef::new(IngredientUnits::IsFractionAllowed).boolean().not_null().default(false))
                    .col(ColumnDef::new(IngredientUnits::IsActive).boolean().not_null().default(true))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-ingredient_units-base_unit_id")
                            .from(IngredientUnits::Table, IngredientUnits::BaseUnitId)
                            .to(IngredientUnits::Table, IngredientUnits::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("recipe_ingredients"))
                    .add_column(ColumnDef::new(Alias::new("unit_id")).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-recipe_ingredients-unit_id")
                            .from_tbl(Alias::new("recipe_ingredients"))
                            .from_col(Alias::new("unit_id"))
                            .to_tbl(IngredientUnits::Table)
                            .to_col(IngredientUnits::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                    )
                    .to_owned(),
            )
            .await?;

        let units = vec![
            ("GRAM", "g", "Gram", "Gramme", "metric", 1.0, true, vec!["gram", "grams"]),
            ("KILOGRAM", "kg", "Kilogram", "Kilogramme", "metric", 1000.0, false, vec!["kilogram", "kilograms"]),
            ("MILLILITER", "ml", "Milliliter", "Millilitre", "metric", 1.0, false, vec!["milliliter", "milliliters"]),
            ("LITER", "l", "Liter", "Litre", "metric", 1000.0, false, vec!["liter", "liters"]),
            ("CUP", "cup", "Cup", "Tasse", "metric", 240.0, true, vec!["cups"]),
            ("PIECE", "pc", "Piece", "Pièce", "other", 1.0, false, vec!["piece", "pieces"]),
        ];

        for (code, symbol, en, fr, sys, factor, fraction, aliases) in units {
            manager.exec_stmt(
                Query::insert()
                    .into_table(IngredientUnits::Table)
                    .columns([
                        IngredientUnits::Code, IngredientUnits::Symbol,
                        IngredientUnits::NameEn, IngredientUnits::NameFr, IngredientUnits::System,
                        IngredientUnits::ConversionFactor, IngredientUnits::IsFractionAllowed
                    ])
                    .values_panic([
                        code.into(), symbol.into(),
                        en.into(), fr.into(), sys.into(),
                        factor.into(), fraction.into()
                    ])
                    .to_owned()
            ).await?;

            let mut match_values = vec![symbol.to_string(), en.to_lowercase()];
            match_values.extend(aliases.into_iter().map(|a| a.to_string()));

            for val in match_values {
                let update_query = format!(
                    "UPDATE recipe_ingredients
             SET unit_id = (SELECT id FROM ingredient_units WHERE code = '{}')
             WHERE unit ILIKE '{}'",
                    code, val
                );
                manager.get_connection().execute_unprepared(&update_query).await?;
            }
        }
        manager.alter_table(
            Table::alter()
                .table(Alias::new("recipe_ingredients"))
                .modify_column(ColumnDef::new(Alias::new("unit_id")).uuid().not_null())
                .drop_column(Alias::new("unit"))
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("recipe_ingredients"))
                    .add_column(ColumnDef::new(Alias::new("unit")).string())
                    .drop_foreign_key(Alias::new("fk-recipe_ingredients-unit_id"))
                    .drop_column(Alias::new("unit_id"))
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(Table::drop().table(IngredientUnits::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum IngredientUnits {
    Table,
    Id,
    Code,
    Symbol,
    NameFr,
    NameEn,
    System,
    BaseUnitId,
    ConversionFactor,
    IsFractionAllowed,
    IsActive,
}