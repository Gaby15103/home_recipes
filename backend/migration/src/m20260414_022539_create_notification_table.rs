use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Templates Table: Maps categories to specific translations
        manager
            .create_table(
                Table::create()
                    .table(NotificationTemplates::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).uuid().primary_key().extra("DEFAULT gen_random_uuid()"))
                    .col(string(NotificationTemplates::Category))
                    .col(string(NotificationTemplates::LanguageCode))
                    .col(string(NotificationTemplates::TitleTemplate))
                    .col(string(NotificationTemplates::MessageTemplate))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-notif-templates-lang")
                            .from(NotificationTemplates::Table, NotificationTemplates::LanguageCode)
                            .to(Alias::new("languages"), Alias::new("code"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx-notif-template-cat-lang")
                            .col(NotificationTemplates::Category)
                            .col(NotificationTemplates::LanguageCode),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Notifications Table: The individual user alerts
        manager
            .create_table(
                Table::create()
                    .table(Notifications::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).uuid().primary_key().extra("DEFAULT gen_random_uuid()"))
                    .col(uuid(Notifications::UserId))
                    .col(uuid_null(Notifications::ActorId))
                    .col(string(Notifications::Category))
                    .col(string(Notifications::Title))
                    .col(string(Notifications::Message))
                    .col(uuid_null(Notifications::TargetId))
                    .col(boolean(Notifications::IsRead).default(false))
                    .col(timestamp_with_time_zone(Notifications::CreatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-notifications-user")
                            .from(Notifications::Table, Notifications::UserId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Notifications::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(NotificationTemplates::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum NotificationTemplates {
    Table,
    Category,
    LanguageCode,
    TitleTemplate,
    MessageTemplate,
}

#[derive(DeriveIden)]
enum Notifications {
    Table,
    UserId,
    ActorId,
    Category,
    Title,
    Message,
    TargetId,
    IsRead,
    CreatedAt,
}