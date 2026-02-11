use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("favorites"))
                    .if_not_exists()
                    // Define Columns
                    .col(ColumnDef::new(Alias::new("user_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("recipe_id")).uuid().not_null())
                    .col(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    // Composite Primary Key (user_id, recipe_id)
                    .primary_key(
                        Index::create()
                            .col(Alias::new("user_id"))
                            .col(Alias::new("recipe_id")),
                    )
                    // Foreign Key: User link
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_favorites_user")
                            .from(Alias::new("favorites"), Alias::new("user_id"))
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // Foreign Key: Recipe link
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_favorites_recipe")
                            .from(Alias::new("favorites"), Alias::new("recipe_id"))
                            .to(Alias::new("recipes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for performance when counting favorites for a specific recipe
        manager
            .create_index(
                Index::create()
                    .name("idx_favorites_recipe_id")
                    .table(Alias::new("favorites"))
                    .col(Alias::new("recipe_id"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("favorites")).to_owned())
            .await
    }
}
