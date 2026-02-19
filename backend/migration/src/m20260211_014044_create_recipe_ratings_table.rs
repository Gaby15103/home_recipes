use crate::sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create the table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("recipe_ratings"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("recipe_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("user_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("rating")).integer().not_null())
                    .col(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    // Composite Primary Key (recipe_id, user_id)
                    .primary_key(
                        Index::create()
                            .col(Alias::new("recipe_id"))
                            .col(Alias::new("user_id")),
                    )
                    // Foreign Keys
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rating_recipe")
                            .from(Alias::new("recipe_ratings"), Alias::new("recipe_id"))
                            .to(Alias::new("recipes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rating_user")
                            .from(Alias::new("recipe_ratings"), Alias::new("user_id"))
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Create the index for recipe lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_ratings_recipe")
                    .table(Alias::new("recipe_ratings"))
                    .col(Alias::new("recipe_id"))
                    .to_owned(),
            )
            .await?;

        // 3. Add the CHECK constraint (rating BETWEEN 1 AND 5)
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE recipe_ratings ADD CONSTRAINT check_rating_range CHECK (rating BETWEEN 1 AND 5)")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("recipe_ratings")).to_owned())
            .await
    }
}