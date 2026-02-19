use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create the 'tags' table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("tags"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(
                        ColumnDef::new(Alias::new("name"))
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Create the 'recipe_tags' junction table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("recipe_tags"))
                    .if_not_exists()
                    // Define columns
                    .col(ColumnDef::new(Alias::new("recipe_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("tag_id")).uuid().not_null())
                    // Composite Primary Key (recipe_id, tag_id)
                    .primary_key(
                        Index::create()
                            .col(Alias::new("recipe_id"))
                            .col(Alias::new("tag_id")),
                    )
                    // Foreign Key for 'recipes'
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe_tags-recipe_id")
                            .from(Alias::new("recipe_tags"), Alias::new("recipe_id"))
                            .to(Alias::new("recipes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // Foreign Key for 'tags'
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe_tags-tag_id")
                            .from(Alias::new("recipe_tags"), Alias::new("tag_id"))
                            .to(Alias::new("tags"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order to respect foreign key constraints
        manager
            .drop_table(Table::drop().table(Alias::new("recipe_tags")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Alias::new("tags")).to_owned())
            .await
    }
}
