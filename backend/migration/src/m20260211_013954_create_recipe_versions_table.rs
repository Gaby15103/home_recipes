use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("recipe_versions"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(ColumnDef::new(Alias::new("recipe_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("data")).json_binary().not_null())
                    .col(ColumnDef::new(Alias::new("edited_by")).uuid()) // Nullable for SET NULL
                    .col(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    // Foreign Key: Link to recipes
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_versions_recipe")
                            .from(Alias::new("recipe_versions"), Alias::new("recipe_id"))
                            .to(Alias::new("recipes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // Foreign Key: Link to users
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_versions_user")
                            .from(Alias::new("recipe_versions"), Alias::new("edited_by"))
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for fast version history retrieval
        manager.create_index(
            Index::create()
                .name("idx_versions_recipe_id")
                .table(Alias::new("recipe_versions"))
                .col(Alias::new("recipe_id"))
                .to_owned()
        ).await?;

        // Index for sorting by date
        manager.create_index(
            Index::create()
                .name("idx_versions_created_at")
                .table(Alias::new("recipe_versions"))
                .col(Alias::new("created_at"))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("recipe_versions")).to_owned())
            .await
    }
}
