use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let units = vec![
            // CODE, SYMBOL, EN, FR, SYSTEM, FACTOR, FRACTION
            ("TABLESPOON", "c. à soupe", "Tablespoon", "Cuillère à soupe", "metric", 15.0, true),
            ("TEASPOON", "c. à thé", "Teaspoon", "Cuillère à thé", "metric", 5.0, true),
            ("CLOVE", "gousse", "Clove", "Gousse", "other", 1.0, false),
            ("BUNCH", "botte", "Bunch", "Botte", "other", 1.0, true),
            ("PINCH", "pincée", "Pinch", "Pincée", "other", 0.0, false),
            ("CAN", "boîte", "Can", "Boîte", "other", 1.0, true),
            ("TO_TASTE", "au goût", "To taste", "Au goût", "other", 0.0, false),
            ("POUND", "lb", "Pound", "Livre", "imperial", 453.59, true),
            ("OUNCE", "oz", "Ounce", "Once", "imperial", 28.35, true),
            ("SLICE", "tranche", "Slice", "Tranche", "other", 1.0, true),
            ("BOX", "paquet", "Box/Packet", "Paquet/Sachet", "other", 1.0, true),
        ];

        for (code, symbol, en, fr, sys, factor, fraction) in units {
            manager.exec_stmt(
                Query::insert()
                    .into_table(AliasIngredientUnits::Table)
                    .columns([
                        AliasIngredientUnits::Code, AliasIngredientUnits::Symbol,
                        AliasIngredientUnits::NameEn, AliasIngredientUnits::NameFr, AliasIngredientUnits::System,
                        AliasIngredientUnits::ConversionFactor, AliasIngredientUnits::IsFractionAllowed
                    ])
                    .values_panic([
                        code.into(), symbol.into(),
                        en.into(), fr.into(), sys.into(),
                        factor.into(), fraction.into()
                    ])
                    .to_owned()
            ).await?;
        }
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Typically you'd delete the specific rows added above
        Ok(())
    }
}

#[derive(Iden)]
enum AliasIngredientUnits {
    #[iden = "ingredient_units"]
    Table,
    Code,
    Symbol,
    NameEn,
    NameFr,
    System,
    ConversionFactor,
    IsFractionAllowed,
}