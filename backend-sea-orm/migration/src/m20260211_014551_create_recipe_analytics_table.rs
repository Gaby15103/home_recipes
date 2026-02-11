use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("recipe_analytics"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(ColumnDef::new(Alias::new("recipe_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("user_id")).uuid()) // Nullable for anonymous views
                    .col(
                        ColumnDef::new(Alias::new("viewed_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    // Foreign Key: Recipe link
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_analytics_recipe")
                            .from(Alias::new("recipe_analytics"), Alias::new("recipe_id"))
                            .to(Alias::new("recipes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // Foreign Key: User link
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_analytics_user")
                            .from(Alias::new("recipe_analytics"), Alias::new("user_id"))
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for counting views per recipe
        manager.create_index(
            Index::create()
                .name("idx_analytics_recipe_id")
                .table(Alias::new("recipe_analytics"))
                .col(Alias::new("recipe_id"))
                .to_owned()
        ).await?;

        // Index for time-based analytics (e.g., views in the last 24h)
        manager.create_index(
            Index::create()
                .name("idx_analytics_viewed_at")
                .table(Alias::new("recipe_analytics"))
                .col(Alias::new("viewed_at"))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("recipe_analytics")).to_owned())
            .await
    }
}
