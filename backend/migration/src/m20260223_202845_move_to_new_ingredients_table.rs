use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        manager.create_table(
            Table::create()
                .table(Alias::new("ingredient_translations_new"))
                .if_not_exists()
                .col(ColumnDef::new(Alias::new("id")).uuid().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(Alias::new("ingredient_id")).uuid().not_null())
                .col(ColumnDef::new(Alias::new("language_code")).string().not_null())
                .col(ColumnDef::new(Alias::new("data")).string().not_null())
                .col(ColumnDef::new(Alias::new("note")).string())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-it-ingredient")
                        .from(Alias::new("ingredient_translations_new"), Alias::new("ingredient_id"))
                        .to(Alias::new("ingredients"), Alias::new("id"))
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-it-language")
                        .from(Alias::new("ingredient_translations_new"), Alias::new("language_code"))
                        .to(Alias::new("languages"), Alias::new("code"))
                        .on_delete(ForeignKeyAction::Restrict),
                )
                .to_owned(),
        ).await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("ingredients"))
                    .add_column(ColumnDef::new(Alias::new("ingredient_group_id")).uuid())
                    .add_column(ColumnDef::new(Alias::new("quantity")).decimal())
                    .add_column(ColumnDef::new(Alias::new("position")).integer())
                    .add_column(ColumnDef::new(Alias::new("unit_id")).uuid())
                    .to_owned(),
            ).await?;

        db.execute_unprepared(r#"
            -- Sync data from join table
            UPDATE ingredients i
            SET
                ingredient_group_id = ri.ingredient_group_id,
                quantity = ri.quantity,
                position = ri.position,
                unit_id = ri.unit_id
            FROM recipe_ingredients ri
            WHERE ri.ingredient_id = i.id;

            -- CRITICAL: Delete ingredients that aren't linked to a recipe.
            -- If we don't do this, the NOT NULL constraint below will fail.
            DELETE FROM ingredients WHERE ingredient_group_id IS NULL;

            -- Populate new translations
            INSERT INTO ingredient_translations_new (ingredient_id, language_code, data, note)
            SELECT
                ri.ingredient_id,
                rit.language_code,
                it_old.name,
                rit.note
            FROM recipe_ingredient_translations rit
            JOIN recipe_ingredients ri ON rit.recipe_ingredient_id = ri.id
            JOIN ingredient_translations it_old ON ri.ingredient_id = it_old.ingredient_id
                AND rit.language_code = it_old.language_code;
        "#).await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("ingredients"))
                    .modify_column(ColumnDef::new(Alias::new("ingredient_group_id")).uuid().not_null())
                    .modify_column(ColumnDef::new(Alias::new("quantity")).decimal().not_null())
                    .modify_column(ColumnDef::new(Alias::new("position")).integer().not_null())
                    .modify_column(ColumnDef::new(Alias::new("unit_id")).uuid().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-ingredients-group_id")
                            .from_tbl(Alias::new("ingredients"))
                            .from_col(Alias::new("ingredient_group_id"))
                            .to_tbl(Alias::new("ingredient_groups"))
                            .to_col(Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-ingredients-unit_id")
                            .from_tbl(Alias::new("ingredients"))
                            .from_col(Alias::new("unit_id"))
                            .to_tbl(Alias::new("ingredient_units"))
                            .to_col(Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict)
                    )
                    .to_owned(),
            ).await?;

        manager.drop_table(Table::drop().table(Alias::new("recipe_ingredient_translations")).to_owned()).await?;
        manager.drop_table(Table::drop().table(Alias::new("ingredient_translations")).to_owned()).await?;
        manager.drop_table(Table::drop().table(Alias::new("recipe_ingredients")).to_owned()).await?;

        db.execute_unprepared(r#"
            ALTER TABLE ingredient_translations_new RENAME TO ingredient_translations;
        "#).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            let db = manager.get_connection();

            // 1. Recreate recipe_ingredients table WITHOUT strict constraints first
            manager.create_table(
                Table::create()
                    .table(Alias::new("recipe_ingredients"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).uuid().primary_key().extra("DEFAULT gen_random_uuid()"))
                    .col(ColumnDef::new(Alias::new("ingredient_group_id")).uuid()) // Nullable for a moment
                    .col(ColumnDef::new(Alias::new("ingredient_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("quantity")).decimal().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("position")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("unit_id")).uuid())
                    .to_owned()
            ).await?;

            // 2. Recreate recipe_ingredient_translations
            manager.create_table(
                Table::create()
                    .table(Alias::new("recipe_ingredient_translations"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).uuid().primary_key().extra("DEFAULT gen_random_uuid()"))
                    .col(ColumnDef::new(Alias::new("recipe_ingredient_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("language_code")).string().not_null())
                    .col(ColumnDef::new(Alias::new("note")).string())
                    .to_owned()
            ).await?;

            // 3. Migrate Data Down
            // IMPORTANT: We filter out rows that don't have a group_id to avoid the crash
            db.execute_unprepared(r#"
        -- Insert back into join table
        INSERT INTO recipe_ingredients (id, ingredient_group_id, ingredient_id, quantity, position, unit_id)
        SELECT id, ingredient_group_id, id, COALESCE(quantity, 0), COALESCE(position, 0), unit_id
        FROM ingredients
        WHERE ingredient_group_id IS NOT NULL;

        -- Insert back into recipe_ingredient_translations
        INSERT INTO recipe_ingredient_translations (recipe_ingredient_id, language_code, note)
        SELECT ri.id, it.language_code, it.note
        FROM ingredient_translations it
        JOIN recipe_ingredients ri ON it.ingredient_id = ri.id;
    "#).await?;

            // 4. Now that data is in, enforce the Not Null and Foreign Keys
            manager.alter_table(
                Table::alter()
                    .table(Alias::new("recipe_ingredients"))
                    .modify_column(ColumnDef::new(Alias::new("ingredient_group_id")).uuid().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-recipe_ingredients-group_id")
                            .from_tbl(Alias::new("recipe_ingredients"))
                            .from_col(Alias::new("ingredient_group_id"))
                            .to_tbl(Alias::new("ingredient_groups"))
                            .to_col(Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned()
            ).await?;

            // 5. Cleanup the columns added to ingredients in the UP migration
            manager.alter_table(
                Table::alter()
                    .table(Alias::new("ingredients"))
                    .drop_column(Alias::new("ingredient_group_id"))
                    .drop_column(Alias::new("quantity"))
                    .drop_column(Alias::new("position"))
                    .drop_column(Alias::new("unit_id"))
                    .to_owned()
            ).await?;

            Ok(())
    }
}