use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("recipe_comments"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(ColumnDef::new(Alias::new("recipe_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("user_id")).uuid()) // Nullable for SET NULL
                    .col(ColumnDef::new(Alias::new("parent_id")).uuid()) // Self-reference
                    .col(ColumnDef::new(Alias::new("content")).string().not_null())
                    .col(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(ColumnDef::new(Alias::new("edited_at")).timestamp_with_time_zone())
                    .col(ColumnDef::new(Alias::new("deleted_at")).timestamp_with_time_zone())
                    // Foreign Key: Recipe link
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_recipe")
                            .from(Alias::new("recipe_comments"), Alias::new("recipe_id"))
                            .to(Alias::new("recipes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // Foreign Key: User link (preserves comment text if user is gone)
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_user")
                            .from(Alias::new("recipe_comments"), Alias::new("user_id"))
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    // Foreign Key: Parent comment (Self-referencing)
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_parent")
                            .from(Alias::new("recipe_comments"), Alias::new("parent_id"))
                            .to(Alias::new("recipe_comments"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for loading recipe comment threads
        manager.create_index(
            Index::create()
                .name("idx_comments_recipe")
                .table(Alias::new("recipe_comments"))
                .col(Alias::new("recipe_id"))
                .to_owned()
        ).await?;

        // Index for finding replies to a specific comment
        manager.create_index(
            Index::create()
                .name("idx_comments_parent")
                .table(Alias::new("recipe_comments"))
                .col(Alias::new("parent_id"))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("recipe_comments")).to_owned())
            .await
    }
}
