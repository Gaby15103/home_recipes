use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create step_group_translations table
        manager.create_table(
            Table::create()
                .table(Alias::new("step_group_translations"))
                .if_not_exists()
                .col(ColumnDef::new(Alias::new("id")).uuid().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(Alias::new("step_group_id")).uuid().not_null())
                .col(ColumnDef::new(Alias::new("language_code")).string().not_null())
                .col(ColumnDef::new(Alias::new("title")).string().not_null())
                .col(ColumnDef::new(Alias::new("created_at")).timestamp().not_null().default(Expr::cust("now()")))
                .col(ColumnDef::new(Alias::new("updated_at")).timestamp().not_null().default(Expr::cust("now()")))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_step_group")
                        .from(Alias::new("step_group_translations"), Alias::new("step_group_id"))
                        .to(Alias::new("step_groups"), Alias::new("id"))
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_language_group")
                        .from(Alias::new("step_group_translations"), Alias::new("language_code"))
                        .to(Alias::new("languages"), Alias::new("code"))
                        .on_delete(ForeignKeyAction::Restrict),
                )
                .index(Index::create().name("unique_step_group_language").col(Alias::new("step_group_id")).col(Alias::new("language_code")).unique())
                .to_owned(),
        ).await?;

        // 2. Create step_translations table
        manager.create_table(
            Table::create()
                .table(Alias::new("step_translations"))
                .if_not_exists()
                .col(ColumnDef::new(Alias::new("id")).uuid().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(Alias::new("step_id")).uuid().not_null())
                .col(ColumnDef::new(Alias::new("language_code")).string().not_null())
                .col(ColumnDef::new(Alias::new("instruction")).string().not_null())
                .col(ColumnDef::new(Alias::new("created_at")).timestamp().not_null().default(Expr::cust("now()")))
                .col(ColumnDef::new(Alias::new("updated_at")).timestamp().not_null().default(Expr::cust("now()")))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_step")
                        .from(Alias::new("step_translations"), Alias::new("step_id"))
                        .to(Alias::new("steps"), Alias::new("id"))
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_language_step")
                        .from(Alias::new("step_translations"), Alias::new("language_code"))
                        .to(Alias::new("languages"), Alias::new("code"))
                        .on_delete(ForeignKeyAction::Restrict),
                )
                .index(Index::create().name("unique_step_language").col(Alias::new("step_id")).col(Alias::new("language_code")).unique())
                .to_owned(),
        ).await?;

        // 3. Move data into translations (using your SQL logic)
        let data_migration_sql = r#"
            INSERT INTO step_group_translations (step_group_id, language_code, title, created_at, updated_at)
            SELECT sg.id, r.original_language_code, sg.title, NOW(), NOW()
            FROM step_groups sg JOIN recipes r ON r.id = sg.recipe_id;

            INSERT INTO step_translations (step_id, language_code, instruction, created_at, updated_at)
            SELECT s.id, r.original_language_code, s.instruction, NOW(), NOW()
            FROM steps s
            JOIN step_groups sg ON sg.id = s.step_group_id
            JOIN recipes r ON r.id = sg.recipe_id;
        "#;
        manager.get_connection().execute_unprepared(data_migration_sql).await?;

        // 4. Drop original columns
        manager.alter_table(Table::alter().table(Alias::new("step_groups")).drop_column(Alias::new("title")).to_owned()).await?;
        manager.alter_table(Table::alter().table(Alias::new("steps")).drop_column(Alias::new("instruction")).to_owned()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Alias::new("step_translations")).to_owned()).await?;
        manager.drop_table(Table::drop().table(Alias::new("step_group_translations")).to_owned()).await
    }
}
