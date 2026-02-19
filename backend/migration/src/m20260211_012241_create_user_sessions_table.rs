use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("sessions"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(ColumnDef::new(Alias::new("user_id")).uuid().not_null())
                    // Security: The actual secret token sent to the user (usually hashed in DB)
                    .col(ColumnDef::new(Alias::new("token")).string().not_null().unique_key())
                    // Metadata for the user to recognize the session
                    .col(ColumnDef::new(Alias::new("user_agent")).string())
                    .col(ColumnDef::new(Alias::new("ip_address")).string())
                    // Status & Expiration
                    .col(ColumnDef::new(Alias::new("is_revoked")).boolean().not_null().default(false))
                    .col(ColumnDef::new(Alias::new("expires_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().default(Expr::cust("now()")))
                    .col(ColumnDef::new(Alias::new("last_active_at")).timestamp_with_time_zone().default(Expr::cust("now()")))
                    // Foreign Key
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sessions-user_id")
                            .from(Alias::new("sessions"), Alias::new("user_id"))
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. Create Indexes for performance
        manager.create_index(
            Index::create()
                .name("idx-sessions-user_id")
                .table(Alias::new("sessions"))
                .col(Alias::new("user_id"))
                .to_owned()
        ).await?;

        manager.create_index(
            Index::create()
                .name("idx-sessions-expires_at")
                .table(Alias::new("sessions"))
                .col(Alias::new("expires_at"))
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("sessions")).to_owned())
            .await
    }
}
