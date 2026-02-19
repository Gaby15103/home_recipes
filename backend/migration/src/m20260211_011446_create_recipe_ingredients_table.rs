use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("recipe_ingredients"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(ColumnDef::new(Alias::new("ingredient_group_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("ingredient_id")).uuid().not_null())
                    .col(
                        ColumnDef::new(Alias::new("quantity"))
                            .decimal_len(10, 2) // Maps to NUMERIC(10, 2)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("unit")).string().not_null())
                    .col(ColumnDef::new(Alias::new("note")).string())
                    .col(ColumnDef::new(Alias::new("position")).integer().not_null())
                    // Foreign Key: Group (Cascade)
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe_ingredients-group_id")
                            .from(Alias::new("recipe_ingredients"), Alias::new("ingredient_group_id"))
                            .to(Alias::new("ingredient_groups"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // Foreign Key: Ingredient (Restrict/No Action by default)
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe_ingredients-ingredient_id")
                            .from(Alias::new("recipe_ingredients"), Alias::new("ingredient_id"))
                            .to(Alias::new("ingredients"), Alias::new("id")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("recipe_ingredients")).to_owned())
            .await
    }
}
