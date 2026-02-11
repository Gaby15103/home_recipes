use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("user_roles")
                    .if_not_exists()
                    .col(ColumnDef::new("user_id").uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_roles-user_id")
                            .from("user_roles", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new("role_id").uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_roles-role_id")
                            .from("user_roles", "role_id")
                            .to("roles", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col("user_id")
                            .col("role_id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("user_roles").to_owned())
            .await
    }
}
