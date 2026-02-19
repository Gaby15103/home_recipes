use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table("roles")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(string("name").string_len(50).not_null().unique_key())
                    .col(string("description").not_null())
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table("roles")
            .columns(["name", "description"])
            .values_panic(["USER".into(), "application user".into()])
            .values_panic(["ADMIN".into(), "administrator".into()])
            .values_panic(["MODERATOR".into(), "moderator".into()])
            .values_panic(["SUPER_ADMIN".into(), "admin with all right".into()])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("roles").to_owned())
            .await
    }
}
