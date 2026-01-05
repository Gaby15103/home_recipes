// @generated automatically by Diesel CLI.

diesel::table! {
    ingredient_groups (id) {
        id -> Uuid,
        recipe_id -> Uuid,
        title -> Text,
        position -> Int4,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    recipe_ingredients (id) {
        id -> Uuid,
        ingredient_group_id -> Uuid,
        ingredient_id -> Uuid,
        quantity -> Numeric,
        unit -> Text,
        note -> Nullable<Text>,
        position -> Int4,
    }
}

diesel::table! {
    recipe_tags (recipe_id, tag_id) {
        recipe_id -> Uuid,
        tag_id -> Uuid,
    }
}

diesel::table! {
    recipes (id) {
        id -> Uuid,
        title -> Text,
        description -> Nullable<Text>,
        image_url -> Text,
        servings -> Int4,
        prep_time_minutes -> Int4,
        cook_time_minutes -> Int4,
        author -> Text,
        author_id -> Nullable<Uuid>,
        is_private -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    step_groups (id) {
        id -> Uuid,
        recipe_id -> Uuid,
        title -> Text,
        position -> Int4,
    }
}

diesel::table! {
    steps (id) {
        id -> Uuid,
        step_group_id -> Uuid,
        position -> Int4,
        instruction -> Text,
        image_url -> Nullable<Text>,
        duration_minutes -> Nullable<Int4>,
    }
}

diesel::table! {
    tags (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Uuid,
        role_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        password_hash -> Text,
        avatar_url -> Nullable<Text>,
        preferences -> Jsonb,
        is_active -> Nullable<Bool>,
        email_verified -> Nullable<Bool>,
        last_login_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(ingredient_groups -> recipes (recipe_id));
diesel::joinable!(recipe_ingredients -> ingredient_groups (ingredient_group_id));
diesel::joinable!(recipe_ingredients -> ingredients (ingredient_id));
diesel::joinable!(recipe_tags -> recipes (recipe_id));
diesel::joinable!(recipe_tags -> tags (tag_id));
diesel::joinable!(recipes -> users (author_id));
diesel::joinable!(step_groups -> recipes (recipe_id));
diesel::joinable!(steps -> step_groups (step_group_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    ingredient_groups,
    ingredients,
    recipe_ingredients,
    recipe_tags,
    recipes,
    roles,
    step_groups,
    steps,
    tags,
    user_roles,
    users,
);
