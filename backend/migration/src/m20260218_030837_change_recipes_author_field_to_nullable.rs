use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("recipes"))
                    .modify_column(
                        ColumnDef::new(Alias::new("author"))
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("recipes"))
                    .modify_column(
                        ColumnDef::new(Alias::new("author"))
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }
}
