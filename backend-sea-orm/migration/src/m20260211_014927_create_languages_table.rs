use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create the 'languages' table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("languages"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("code"))
                            .string_len(10) // Maps to VARCHAR(10)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("name")).string().not_null())
                    .col(ColumnDef::new(Alias::new("native_name")).string().not_null())
                    .col(
                        ColumnDef::new(Alias::new("is_active"))
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Alias::new("is_default"))
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Seed the initial data
        let insert_sql = r#"
            INSERT INTO languages (code, name, native_name, is_default)
            VALUES ('en', 'English', 'English', TRUE),
                   ('fr', 'French', 'Français', FALSE),
                   ('fr-CA', 'Canadian French', 'Français (Canada)', FALSE);
        "#;

        manager
            .get_connection()
            .execute_unprepared(insert_sql)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS languages CASCADE")
            .await?;

        Ok(())
    }
}
