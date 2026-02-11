use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("step_groups"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(ColumnDef::new(Alias::new("recipe_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("title")).string().not_null())
                    .col(ColumnDef::new(Alias::new("position")).integer().not_null())
                    // Foreign Key constraint
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-step_groups-recipe_id")
                            .from(Alias::new("step_groups"), Alias::new("recipe_id"))
                            .to(Alias::new("recipes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // UNIQUE (recipe_id, position)
                    .index(
                        Index::create()
                            .name("idx-step_groups-recipe-position-unique")
                            .table(Alias::new("step_groups"))
                            .col(Alias::new("recipe_id"))
                            .col(Alias::new("position"))
                            .unique(),
                    )
                    
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("step_groups")).to_owned())
            .await
    }
}
