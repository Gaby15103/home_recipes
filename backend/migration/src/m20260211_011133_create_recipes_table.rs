use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("recipes"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(ColumnDef::new(Alias::new("title")).string().not_null())
                    .col(ColumnDef::new(Alias::new("description")).string())
                    .col(ColumnDef::new(Alias::new("image_url")).string().not_null())
                    .col(ColumnDef::new(Alias::new("servings")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("prep_time_minutes")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("cook_time_minutes")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("author")).string().not_null())
                    .col(ColumnDef::new(Alias::new("author_id")).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipes-author_id")
                            .from(Alias::new("recipes"), Alias::new("author_id"))
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Alias::new("is_private"))
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Alias::new("updated_at"))
                            .timestamp_with_time_zone()
                            .default(Expr::cust("now()")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("recipes")).to_owned())
            .await
    }
}
