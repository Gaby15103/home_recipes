use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("users"))
                    .add_column(ColumnDef::new(Alias::new("two_factor_secret")).string())
                    .add_column(ColumnDef::new(Alias::new("two_factor_recovery_codes")).json_binary())
                    .add_column(ColumnDef::new(Alias::new("two_factor_confirmed_at")).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("users"))
                    .drop_column(Alias::new("two_factor_secret"))
                    .drop_column(Alias::new("two_factor_recovery_codes"))
                    .drop_column(Alias::new("two_factor_confirmed_at"))
                    .to_owned(),
            )
            .await
    }
}
