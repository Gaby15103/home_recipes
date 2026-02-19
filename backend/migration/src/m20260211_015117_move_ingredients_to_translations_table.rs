use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create ingredient_group_translations
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("ingredient_group_translations"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).uuid().primary_key().extra("DEFAULT gen_random_uuid()"))
                    .col(ColumnDef::new(Alias::new("ingredient_group_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("language_code")).string().not_null())
                    .col(ColumnDef::new(Alias::new("title")).string().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp().not_null().default(Expr::cust("now()")))
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp().not_null().default(Expr::cust("now()")))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ingredient_group")
                            .from(Alias::new("ingredient_group_translations"), Alias::new("ingredient_group_id"))
                            .to(Alias::new("ingredient_groups"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_language_ing_group")
                            .from(Alias::new("ingredient_group_translations"), Alias::new("language_code"))
                            .to(Alias::new("languages"), Alias::new("code"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .index(Index::create().name("unique_ingredient_group_language").col(Alias::new("ingredient_group_id")).col(Alias::new("language_code")).unique())
                    .to_owned(),
            )
            .await?;

        // 2. Create ingredient_translations
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("ingredient_translations"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).uuid().primary_key().extra("DEFAULT gen_random_uuid()"))
                    .col(ColumnDef::new(Alias::new("ingredient_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("language_code")).string().not_null())
                    .col(ColumnDef::new(Alias::new("name")).string().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp().not_null().default(Expr::cust("now()")))
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp().not_null().default(Expr::cust("now()")))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ingredient")
                            .from(Alias::new("ingredient_translations"), Alias::new("ingredient_id"))
                            .to(Alias::new("ingredients"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_language_ing")
                            .from(Alias::new("ingredient_translations"), Alias::new("language_code"))
                            .to(Alias::new("languages"), Alias::new("code"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .index(Index::create().name("unique_ingredient_language").col(Alias::new("ingredient_id")).col(Alias::new("language_code")).unique())
                    .to_owned(),
            )
            .await?;

        // 3. Create recipe_ingredient_translations
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("recipe_ingredient_translations"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).uuid().primary_key().extra("DEFAULT gen_random_uuid()"))
                    .col(ColumnDef::new(Alias::new("recipe_ingredient_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("language_code")).string().not_null())
                    .col(ColumnDef::new(Alias::new("note")).string())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp().not_null().default(Expr::cust("now()")))
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp().not_null().default(Expr::cust("now()")))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_recipe_ingredient")
                            .from(Alias::new("recipe_ingredient_translations"), Alias::new("recipe_ingredient_id"))
                            .to(Alias::new("recipe_ingredients"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_language_rec_ing")
                            .from(Alias::new("recipe_ingredient_translations"), Alias::new("language_code"))
                            .to(Alias::new("languages"), Alias::new("code"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .index(Index::create().name("unique_recipe_ingredient_language").col(Alias::new("recipe_ingredient_id")).col(Alias::new("language_code")).unique())
                    .to_owned(),
            )
            .await?;

        // 4. Data Migration
        let data_migration_sql = r#"
            INSERT INTO ingredient_group_translations (ingredient_group_id, language_code, title)
            SELECT ig.id, r.original_language_code, ig.title
            FROM ingredient_groups ig JOIN recipes r ON r.id = ig.recipe_id;

            INSERT INTO ingredient_translations (ingredient_id, language_code, name)
            SELECT i.id, (SELECT code FROM languages WHERE is_default = TRUE LIMIT 1), i.name
            FROM ingredients i;

            INSERT INTO recipe_ingredient_translations (recipe_ingredient_id, language_code, note)
            SELECT ri.id, r.original_language_code, ri.note
            FROM recipe_ingredients ri
            JOIN ingredient_groups ig ON ig.id = ri.ingredient_group_id
            JOIN recipes r ON r.id = ig.recipe_id
            WHERE ri.note IS NOT NULL;
        "#;
        manager.get_connection().execute_unprepared(data_migration_sql).await?;

        // 5. Drop original columns
        manager.alter_table(Table::alter().table(Alias::new("ingredient_groups")).drop_column(Alias::new("title")).to_owned()).await?;
        manager.alter_table(Table::alter().table(Alias::new("ingredients")).drop_column(Alias::new("name")).to_owned()).await?;
        manager.alter_table(Table::alter().table(Alias::new("recipe_ingredients")).drop_column(Alias::new("note")).to_owned()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Alias::new("recipe_ingredient_translations")).to_owned()).await?;
        manager.drop_table(Table::drop().table(Alias::new("ingredient_translations")).to_owned()).await?;
        manager.drop_table(Table::drop().table(Alias::new("ingredient_group_translations")).to_owned()).await
    }
}
