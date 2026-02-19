use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("steps"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(ColumnDef::new(Alias::new("step_group_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("position")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("instruction")).string().not_null())
                    .col(ColumnDef::new(Alias::new("image_url")).string())
                    .col(ColumnDef::new(Alias::new("duration_minutes")).integer())
                    // Foreign Key constraint
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-steps-step_group_id")
                            .from(Alias::new("steps"), Alias::new("step_group_id"))
                            .to(Alias::new("step_groups"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx-steps-group-position-unique")
                            .table(Alias::new("steps"))
                            .col(Alias::new("step_group_id"))
                            .col(Alias::new("position"))
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("steps")).to_owned())
            .await
    }
}
