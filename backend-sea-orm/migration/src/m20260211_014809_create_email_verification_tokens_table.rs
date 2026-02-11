use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("email_verification_tokens"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(ColumnDef::new(Alias::new("user_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("token")).uuid().not_null())
                    .col(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    // Foreign Key: Link to users
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_email_user")
                            .from(Alias::new("email_verification_tokens"), Alias::new("user_id"))
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for finding tokens by user
        manager.create_index(
            Index::create()
                .name("idx_email_user_id")
                .table(Alias::new("email_verification_tokens"))
                .col(Alias::new("user_id"))
                .to_owned()
        ).await?;

        // Index for cleanup of old tokens
        manager.create_index(
            Index::create()
                .name("idx_email_created_at")
                .table(Alias::new("email_verification_tokens"))
                .col(Alias::new("created_at"))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("email_verification_tokens")).to_owned())
            .await
    }
}
