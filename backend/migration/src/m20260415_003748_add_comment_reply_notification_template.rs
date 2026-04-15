use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert_reply = Query::insert()
            .into_table(NotificationTemplates::Table)
            .columns([
                NotificationTemplates::Category,
                NotificationTemplates::LanguageCode,
                NotificationTemplates::TitleTemplate,
                NotificationTemplates::MessageTemplate,
            ])
            .values_panic([
                "comment_reply".into(),
                "en".into(),
                "New Reply".into(),
                "{actor} replied to your comment on {recipe_title}: {comment_preview}".into(),
            ])
            .to_owned();

        manager.exec_stmt(insert_reply).await?;

        let update_comment = Query::update()
            .table(NotificationTemplates::Table)
            .values([
                (
                    NotificationTemplates::MessageTemplate,
                    "{actor} commented on your recipe {recipe_title}: {comment_preview}".into()
                ),
            ])
            .and_where(Expr::col(NotificationTemplates::Category).eq("recipe_comment"))
            .and_where(Expr::col(NotificationTemplates::LanguageCode).eq("en"))
            .to_owned();

        manager.exec_stmt(update_comment).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let revert_comment = Query::update()
            .table(NotificationTemplates::Table)
            .values([
                (
                    NotificationTemplates::MessageTemplate,
                    "{actor} commented on your recipe: {comment_preview}".into()
                ),
            ])
            .and_where(Expr::col(NotificationTemplates::Category).eq("recipe_comment"))
            .and_where(Expr::col(NotificationTemplates::LanguageCode).eq("en"))
            .to_owned();

        manager.exec_stmt(revert_comment).await?;

        let delete_reply = Query::delete()
            .from_table(NotificationTemplates::Table)
            .and_where(Expr::col(NotificationTemplates::Category).eq("comment_reply"))
            .to_owned();

        manager.exec_stmt(delete_reply).await
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