use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";")
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("users")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(string("email").string_len(255).not_null().unique_key())
                    .col(string("username").not_null().string_len(100))
                    .col(string("first_name").not_null().string_len(100))
                    .col(string("last_name").not_null().string_len(100))
                    .col(string("password_hash").not_null())
                    .col(string("avatar_url").not_null())
                    .col(
                        json_binary("preferences")
                            .not_null()
                            .extra("DEFAULT '{\"language\": \"fr\", \"theme\": \"Dark\"}'::jsonb"),
                    )
                    .col(boolean("is_active").default(false))
                    .col(boolean("email_verified").default(false))
                    .col(timestamp("last_login_at").null())
                    .col(timestamp("created_at").not_null().extra("DEFAULT NOW()"))
                    .col(timestamp("updated_at").not_null().extra("DEFAULT NOW()"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("users").to_owned())
            .await.expect("TODO: panic message");

        manager
            .get_connection()
            .execute_unprepared("DROP EXTENSION IF EXISTS \"uuid-ossp\";")
            .await?;
        Ok(())
    }
}
