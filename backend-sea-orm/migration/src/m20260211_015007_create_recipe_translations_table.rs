use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create recipe_translations table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("recipe_translations"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).uuid().primary_key().extra("DEFAULT gen_random_uuid()"))
                    .col(ColumnDef::new(Alias::new("recipe_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("language_code")).string().not_null())
                    .col(ColumnDef::new(Alias::new("title")).string().not_null())
                    .col(ColumnDef::new(Alias::new("description")).string().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp().not_null().default(Expr::cust("now()")))
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp().not_null().default(Expr::cust("now()")))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_recipe")
                            .from(Alias::new("recipe_translations"), Alias::new("recipe_id"))
                            .to(Alias::new("recipes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_language")
                            .from(Alias::new("recipe_translations"), Alias::new("language_code"))
                            .to(Alias::new("languages"), Alias::new("code"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .index(
                        Index::create()
                            .name("unique_recipe_language")
                            .col(Alias::new("recipe_id"))
                            .col(Alias::new("language_code"))
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Alter recipes table (Add original_language_code)
        // We use raw SQL for IF NOT EXISTS column logic which builder doesn't support natively
        manager.get_connection().execute_unprepared(r#"
            ALTER TABLE recipes ADD COLUMN IF NOT EXISTS original_language_code TEXT NOT NULL DEFAULT 'en';
            ALTER TABLE recipes ADD CONSTRAINT fk_recipe_original_language
                FOREIGN KEY (original_language_code) REFERENCES languages (code);
        "#).await?;

        // 3. Move existing text into translations
        manager.get_connection().execute_unprepared(r#"
            INSERT INTO recipe_translations (recipe_id, language_code, title, description, created_at, updated_at)
            SELECT id, original_language_code, title, description, created_at, created_at
            FROM recipes;
        "#).await?;

        // 4. Remove text from recipes table
        manager.alter_table(
            Table::alter()
                .table(Alias::new("recipes"))
                .drop_column(Alias::new("title"))
                .drop_column(Alias::new("description"))
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Rollback is complex: would require adding columns back and moving data from translations
        manager.drop_table(Table::drop().table(Alias::new("recipe_translations")).to_owned()).await?;
        Ok(())
    }
}
