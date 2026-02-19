use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Raw SQL to create the trigger function
        db.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION update_updated_at_column()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = CURRENT_TIMESTAMP;
                RETURN NEW;
            END;
            $$ language 'plpgsql';
            "#,
        )
            .await?;

        let tables = vec![
            "ingredient_group_translations",
            "ingredient_translations",
            "recipe_comments",
            "recipe_ingredient_translations",
            "recipe_translations",
            "recipes",
            "step_group_translations",
            "step_translations",
            "users",
        ];

        for table in tables {
            create_updated_at_trigger(manager, table).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tables = vec![
            "ingredient_group_translations",
            "ingredient_translations",
            "recipe_comments",
            "recipe_ingredient_translations",
            "recipe_translations",
            "recipes",
            "step_group_translations",
            "step_translations",
            "users",
        ];

        for table in tables {
            drop_updated_at_trigger(manager, table).await?;
        }
        Ok(())
    }
}

pub async fn create_updated_at_trigger(
    manager: &SchemaManager<'_>,
    table_name: &str,
) -> Result<(), DbErr> {
    let db = manager.get_connection();

    // We use a unique name for each trigger based on the table
    let sql = format!(
        r#"
        CREATE TRIGGER trg_update_{table_name}_updated_at
        BEFORE UPDATE ON {table_name}
        FOR EACH ROW
        EXECUTE FUNCTION update_updated_at_column();
        "#
    );

    db.execute_unprepared(&sql).await?;
    Ok(())
}
pub async fn drop_updated_at_trigger(
    manager: &SchemaManager<'_>,
    table_name: &str,
) -> Result<(), DbErr> {
    let db = manager.get_connection();

    // DROP TRIGGER syntax in Postgres requires the table name
    let sql = format!(
        "DROP TRIGGER IF EXISTS trg_update_{table_name}_updated_at ON {table_name};"
    );

    db.execute_unprepared(&sql).await?;
    Ok(())
}