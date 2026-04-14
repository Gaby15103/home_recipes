use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(NotificationTemplates::Table)
            .columns([
                NotificationTemplates::Category,
                NotificationTemplates::LanguageCode,
                NotificationTemplates::TitleTemplate,
                NotificationTemplates::MessageTemplate,
            ])
            .values_panic([
                "recipe_favorite".into(),
                "en".into(),
                "New Favorite!".into(),
                "{actor} added your recipe to their favorites.".into(),
            ])
            .values_panic([
                "recipe_comment".into(),
                "en".into(),
                "New Comment".into(),
                "{actor} commented on your recipe: {comment_preview}".into(),
            ])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::delete()
                    .from_table(NotificationTemplates::Table)
                    .and_where(
                        Expr::col(NotificationTemplates::Category)
                            .is_in(["recipe_favorite", "recipe_comment"]),
                    )
                    .to_owned(),
            )
            .await
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