pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20260211_005451_create_roles_table;
mod m20260211_010656_create_user_roles_table;
mod m20260211_011133_create_recipes_table;
mod m20260211_011228_create_ingredients_table;
mod m20260211_011348_create_ingredient_groups_table;
mod m20260211_011446_create_recipe_ingredients_table;
mod m20260211_011519_create_tags_table;
mod m20260211_011704_create_step_groups_table;
mod m20260211_011731_create_steps_table;
mod m20260211_012241_create_user_sessions_table;
mod m20260211_013915_create_favorites_table;
mod m20260211_013954_create_recipe_versions_table;
mod m20260211_014044_create_recipe_ratings_table;
mod m20260211_014516_create_recipe_comments_table;
mod m20260211_014551_create_recipe_analytics_table;
mod m20260211_014627_add_two_factor_to_users_table;
mod m20260211_014659_add_two_factor_token_to_users_table;
mod m20260211_014728_add_two_factor_expire_at_to_users_table;
mod m20260211_014809_create_email_verification_tokens_table;
mod m20260211_014842_add_seeds;
mod m20260211_014927_create_languages_table;
mod m20260211_015007_create_recipe_translations_table;
mod m20260211_015044_move_steps_to_translations_table;
mod m20260211_015117_move_ingredients_to_translations_table;
mod m20260212_014801_create_password_reset_tokens;
mod m20260217_014849_add_default_language_to_ingredients;
mod m20260218_030837_change_recipes_author_field_to_nullable;
mod m20260219_023250_add_automatic_update_at_to_all_Table_with_update_at;
mod m20260220_002434_create_ingredient_units_table;
mod m20260223_202845_move_to_new_ingredients_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20260211_005451_create_roles_table::Migration),
            Box::new(m20260211_010656_create_user_roles_table::Migration),
            Box::new(m20260211_011133_create_recipes_table::Migration),
            Box::new(m20260211_011228_create_ingredients_table::Migration),
            Box::new(m20260211_011348_create_ingredient_groups_table::Migration),
            Box::new(m20260211_011446_create_recipe_ingredients_table::Migration),
            Box::new(m20260211_011519_create_tags_table::Migration),
            Box::new(m20260211_011704_create_step_groups_table::Migration),
            Box::new(m20260211_011731_create_steps_table::Migration),
            Box::new(m20260211_012241_create_user_sessions_table::Migration),
            Box::new(m20260211_013915_create_favorites_table::Migration),
            Box::new(m20260211_013954_create_recipe_versions_table::Migration),
            Box::new(m20260211_014044_create_recipe_ratings_table::Migration),
            Box::new(m20260211_014516_create_recipe_comments_table::Migration),
            Box::new(m20260211_014551_create_recipe_analytics_table::Migration),
            Box::new(m20260211_014627_add_two_factor_to_users_table::Migration),
            Box::new(m20260211_014659_add_two_factor_token_to_users_table::Migration),
            Box::new(m20260211_014728_add_two_factor_expire_at_to_users_table::Migration),
            Box::new(m20260211_014809_create_email_verification_tokens_table::Migration),
            Box::new(m20260211_014842_add_seeds::Migration),
            Box::new(m20260211_014927_create_languages_table::Migration),
            Box::new(m20260211_015007_create_recipe_translations_table::Migration),
            Box::new(m20260211_015044_move_steps_to_translations_table::Migration),
            Box::new(m20260211_015117_move_ingredients_to_translations_table::Migration),
            Box::new(m20260212_014801_create_password_reset_tokens::Migration),
            Box::new(m20260217_014849_add_default_language_to_ingredients::Migration),
            Box::new(m20260218_030837_change_recipes_author_field_to_nullable::Migration),
            Box::new(m20260219_023250_add_automatic_update_at_to_all_Table_with_update_at::Migration),
            Box::new(m20260220_002434_create_ingredient_units_table::Migration),
            Box::new(m20260223_202845_move_to_new_ingredients_table::Migration),
        ]
    }
}
