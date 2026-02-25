use std::collections::HashMap;
use crate::dto::comment_dto::{CommentDto, CreateCommentDto};
use crate::dto::ingredient_group_dto::IngredientGroupViewDto;
use crate::dto::recipe_dto::{CreateRecipeInput, EditRecipeInput, RecipeEditorDto, RecipeFilter, RecipeFilterByPage, RecipeViewDto};
use crate::dto::step_group_dto::StepGroupViewDto;
use crate::dto::tag_dto::{InputTag, TagDto};
use crate::dto::user_dto::UserResponseDto;
use crate::errors::Error;
use crate::repositories::{ingredient_group_repository, role_repository, step_group_repository, tag_repository};
use entity::{favorites, ingredient_groups, ingredient_translations, ingredients, recipe_analytics, recipe_comments, recipe_ratings, recipe_tags, recipe_translations, recipe_versions, recipes, users};
use migration::JoinType;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, DeleteResult, FromQueryResult, PaginatorTrait, SelectExt, Set, TransactionTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{ExprTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait};
use serde_json::json;
use std::ops::Deref;
use chrono::{DateTime, Utc};
use futures_util::TryFutureExt;
use uuid::Uuid;
use crate::dto::recipe_rating_dto::RecipeRatingDto;
use crate::dto::recipe_version_dto::RecipeVersionDto;

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<recipes::Model>, Error> {
    recipes::Entity::find().all(db).await.map_err(Error::from)
}
pub async fn find_by_query(
    db: &DatabaseConnection,
    filter: RecipeFilter,
    lang_code: &str,
) -> Result<Option<Vec<recipes::Model>>, Error> {
    let mut query = recipes::Entity::find();

    // 1. Correct Scope Handling
    // Scope = true (Admin/All), Scope = false (Public Only)
    if !filter.scope {
        // If scope is false, we strictly only show public recipes
        query = query.filter(recipes::Column::IsPrivate.eq(false));
    }

    // 2. Search Text (Only join and filter if search is NOT empty)
    if let Some(s) = &filter.search {
        if !s.trim().is_empty() {
            let pattern = format!("%{}%", s);
            query = query
                .join(
                    JoinType::LeftJoin,
                    recipes::Relation::RecipeTranslations.def(),
                )
                .filter(
                    recipe_translations::Column::LanguageCode.eq(lang_code).and(
                        recipe_translations::Column::Title
                            .like(&pattern)
                            .or(recipe_translations::Column::Description.like(&pattern)),
                    ),
                );
        }
    }

    if let Some(ingredients_list) = &filter.ingredient {
        if !ingredients_list.is_empty() {
            query = query
                .join(JoinType::InnerJoin, recipes::Relation::IngredientGroups.def())
                .join(JoinType::InnerJoin, ingredient_groups::Relation::Ingredients.def())
                .join(JoinType::InnerJoin, ingredient_translations::Relation::Ingredients.def())
                .filter(ingredient_translations::Column::LanguageCode.eq(lang_code));

            // Chain OR conditions for each ingredient string in the array
            let mut condition = sea_orm::Condition::any();
            for ing in ingredients_list {
                if !ing.trim().is_empty() {
                    condition = condition.add(ingredient_translations::Column::Data.like(format!("%{}%", ing)));
                }
            }
            query = query.filter(condition);
        }
    }

    // 4. Grouping & Execution
    // Grouping by ID is vital to avoid duplicates from the joins
    query = query
        .group_by(recipes::Column::Id)
        .order_by_desc(recipes::Column::CreatedAt);

    let results = query.all(db).await.map_err(|e| {
        eprintln!("Database Error: {:?}", e);
        Error::InternalServerError
    })?;

    if results.is_empty() {
        Ok(None)
    } else {
        Ok(Some(results))
    }
}

pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<recipes::Model, Error> {
    recipes::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(json!({"error":"Recipe not found"})))
}
pub async fn create(
    db: &DatabaseConnection,
    new_recipe: CreateRecipeInput,
    preferred_language: &str,
) -> Result<RecipeViewDto, Error> {
    let pref_lang = preferred_language.to_string();
    db.transaction::<_, RecipeViewDto, Error>(|txn| {
        Box::pin(async move {
            // 1. Insert the Base Recipe
            let recipe_model = recipes::ActiveModel {
                image_url: Set(new_recipe.image_url),
                author_id: Set(new_recipe.author_id),
                author: Set(new_recipe.author),
                servings: Set(new_recipe.servings),
                prep_time_minutes: Set(new_recipe.prep_time_minutes),
                cook_time_minutes: Set(new_recipe.cook_time_minutes),
                is_private: Set(new_recipe.is_private),
                original_language_code: Set(new_recipe.primary_language),
                ..Default::default()
            }
            .insert(txn)
            .await?;

            // 2. Insert Translations
            for trans in new_recipe.translations {
                recipe_translations::ActiveModel {
                    recipe_id: Set(recipe_model.id),
                    language_code: Set(trans.language_code),
                    title: Set(trans.title),
                    description: Set(trans.description),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
            }
            let inserted_tags: Vec<TagDto> =
                tag_repository::find_or_create_tags(txn, new_recipe.tags, recipe_model.id).await?;
            let inserted_ingredient_group: Vec<IngredientGroupViewDto> =
                ingredient_group_repository::create_multiple(
                    &txn,
                    recipe_model.id.clone(),
                    new_recipe.ingredient_groups,
                    recipe_model.original_language_code.deref(),
                )
                .await?;
            let inserted_step_group: Vec<StepGroupViewDto> =
                step_group_repository::create_multiple(
                    &txn,
                    recipe_model.id.clone(),
                    new_recipe.step_groups,
                    recipe_model.original_language_code.deref(),
                )
                .await?;

            let translations = recipe_translations::Entity::find()
                .filter(recipe_translations::Column::RecipeId.eq(recipe_model.id))
                .all(txn)
                .await?;

            let main_trans = translations
                .iter()
                .find(|t| t.language_code == pref_lang)
                .or_else(|| {
                    translations
                        .iter()
                        .find(|t| t.language_code == recipe_model.original_language_code)
                })
                .cloned()
                .ok_or(Error::InternalServerError)?;

            Ok(RecipeViewDto::build(
                recipe_model,
                main_trans,
                inserted_tags,
                inserted_ingredient_group,
                inserted_step_group,
            ))
        })
    })
    .await
    .map_err(|e| e.into())
}
pub async fn find_by_query_by_page(
    db: &DatabaseConnection,
    filter: RecipeFilterByPage,
    lang_code: &str,
) -> Result<Option<Vec<recipes::Model>>, Error> {
    let mut query = recipes::Entity::find();

    if let Some(filter) = filter.filters {
        query = query.filter(recipes::Column::IsPrivate.eq(!filter.scope));
        if let Some(s) = &filter.search {
            if !s.trim().is_empty() {
                let pattern = format!("%{}%", s);
                query = query
                    .join(
                        JoinType::LeftJoin,
                        recipes::Relation::RecipeTranslations.def(),
                    )
                    .filter(
                        recipe_translations::Column::LanguageCode.eq(lang_code).and(
                            recipe_translations::Column::Title
                                .like(&pattern)
                                .or(recipe_translations::Column::Description.like(&pattern)),
                        ),
                    );
            }
        }

        if let Some(ingredients_list) = &filter.ingredient {
            if !ingredients_list.is_empty() {
                query = query
                    .join(JoinType::InnerJoin, recipes::Relation::IngredientGroups.def())
                    .join(JoinType::InnerJoin, ingredient_groups::Relation::Ingredients.def())
                    .join(JoinType::InnerJoin, ingredient_translations::Relation::Ingredients.def())
                    .filter(ingredient_translations::Column::LanguageCode.eq(lang_code));

                // Chain OR conditions for each ingredient string in the array
                let mut condition = sea_orm::Condition::any();
                for ing in ingredients_list {
                    if !ing.trim().is_empty() {
                        condition = condition.add(ingredient_translations::Column::Data.like(format!("%{}%", ing)));
                    }
                }
                query = query.filter(condition);
            }
        }
    }

    query = query
        .group_by(recipes::Column::Id)
        .order_by_desc(recipes::Column::CreatedAt);

    let page = filter.page.unwrap_or(1);
    let per_page = filter.per_page.unwrap_or(10);

    let paginator = query.paginate(db, per_page as u64);
    let results = paginator.fetch_page((page - 1) as u64).await?;

    if results.is_empty() {
        Ok(None)
    } else {
        Ok(Some(results))
    }
}

pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, Error> {
    let result = recipes::Entity::delete_by_id(id).exec(db).await?;
    Ok(result)
}
pub async fn get_favorites(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<recipes::Model>, Error> {
    let recipes = recipes::Entity::find()
        .join(JoinType::InnerJoin, recipes::Relation::Favorites.def())
        .filter(favorites::Column::UserId.eq(user_id))
        .all(db)
        .await?;
    Ok(recipes)
}
pub async fn update(
    db: &DatabaseConnection,
    updated_recipe: EditRecipeInput,
    recipe_id: Uuid,
    lang_code: &str,
) -> Result<(), Error> {
    let lang_code_owned = lang_code.to_string();

    db.transaction::<_, (), Error>(|txn| {
        let lang_code = lang_code_owned.clone();

        Box::pin(async move {
            let original_recipe = recipes::Entity::find_by_id(recipe_id)
                .one(txn)
                .await?
                .ok_or(Error::NotFound(json!({"error": "Recipe not found"})))?;

            let mut active_model: recipes::ActiveModel = original_recipe.clone().into();
            let mut base_changed = false;

            if original_recipe.image_url != updated_recipe.image_url {
                active_model.image_url = Set(updated_recipe.image_url);
                base_changed = true;
            }
            if original_recipe.servings != updated_recipe.servings {
                active_model.servings = Set(updated_recipe.servings);
                base_changed = true;
            }
            if original_recipe.prep_time_minutes != updated_recipe.prep_time_minutes {
                active_model.prep_time_minutes = Set(updated_recipe.prep_time_minutes);
                base_changed = true;
            }
            if original_recipe.cook_time_minutes != updated_recipe.cook_time_minutes {
                active_model.cook_time_minutes = Set(updated_recipe.cook_time_minutes);
                base_changed = true;
            }
            if original_recipe.is_private != updated_recipe.is_private {
                active_model.is_private = Set(updated_recipe.is_private);
                base_changed = true;
            }

            let recipe_model = if base_changed {
                active_model.update(txn).await?
            } else {
                original_recipe
            };

            let incoming_ids: Vec<Uuid> = updated_recipe
                .translations
                .iter()
                .filter_map(|t| t.id)
                .collect();

            recipe_translations::Entity::delete_many()
                .filter(recipe_translations::Column::RecipeId.eq(recipe_id))
                .filter(recipe_translations::Column::Id.is_not_in(incoming_ids))
                .exec(txn)
                .await?;

            for trans_input in updated_recipe.translations {
                match trans_input.id {
                    Some(existing_id) => {
                        let existing_trans = recipe_translations::Entity::find_by_id(existing_id)
                            .one(txn)
                            .await?
                            .ok_or(Error::NotFound(json!({"error": "Translation not found"})))?;

                        if existing_trans.title != trans_input.title
                            || existing_trans.description != trans_input.description
                            || existing_trans.language_code != trans_input.language_code
                        {
                            let mut trans_active: recipe_translations::ActiveModel =
                                existing_trans.into();
                            trans_active.language_code = Set(trans_input.language_code);
                            trans_active.title = Set(trans_input.title);
                            trans_active.description = Set(trans_input.description);
                            trans_active.update(txn).await?;
                        }
                    }
                    None => {
                        recipe_translations::ActiveModel {
                            id: Set(Uuid::new_v4()),
                            recipe_id: Set(recipe_id),
                            language_code: Set(trans_input.language_code),
                            title: Set(trans_input.title),
                            description: Set(trans_input.description),
                            ..Default::default()
                        }
                        .insert(txn)
                        .await?;
                    }
                }
            }

            let incoming_existing_tag_ids: Vec<Uuid> = updated_recipe
                .tags
                .iter()
                .filter_map(|t| {
                    if let InputTag::Existing { id } = t {
                        Some(*id)
                    } else {
                        None
                    }
                })
                .collect();

            recipe_tags::Entity::delete_many()
                .filter(recipe_tags::Column::RecipeId.eq(recipe_id))
                .filter(recipe_tags::Column::TagId.is_not_in(incoming_existing_tag_ids))
                .exec(txn)
                .await?;

            let inserted_tags =
                tag_repository::find_or_create_tags(txn, updated_recipe.tags, recipe_id).await?;

            let incoming_group_ids: Vec<Uuid> = updated_recipe
                .ingredient_groups
                .iter()
                .filter_map(|g| g.id)
                .collect();

            ingredient_groups::Entity::delete_many()
                .filter(ingredient_groups::Column::RecipeId.eq(recipe_id))
                .filter(ingredient_groups::Column::Id.is_not_in(incoming_group_ids))
                .exec(txn)
                .await?;

            ingredient_group_repository::update(txn, recipe_id, updated_recipe.ingredient_groups)
                .await?;

            step_group_repository::update(txn, recipe_id, updated_recipe.step_groups).await?;

            Ok(())
        })
    })
    .await
    .map_err(|e| e.into())
}
pub async fn get_analytics(db: &DatabaseConnection, recipe_id: Uuid) -> Result<u64, Error> {
    let count = recipe_analytics::Entity::find()
        .filter(recipe_analytics::Column::RecipeId.eq(recipe_id))
        .count(db)
        .await?;
    Ok(count)
}
pub async fn add_view(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<(), Error> {
    if let Some(user_id) = user_id {
        recipe_analytics::ActiveModel {
            recipe_id: Set(recipe_id),
            user_id: Set(Option::from(user_id)),
            ..Default::default()
        }
        .insert(db)
        .await?;
    } else {
        recipe_analytics::ActiveModel {
            recipe_id: Set(recipe_id),
            ..Default::default()
        }
        .insert(db)
        .await?;
    };
    Ok(())
}

pub async fn toogle_favorite(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Uuid,
) -> Result<bool, Error> {
    let favorited = favorites::Entity::find()
        .filter(favorites::Column::RecipeId.eq(recipe_id))
        .filter(favorites::Column::UserId.eq(user_id))
        .exists(db)
        .await?;
    if favorited {
        let res = favorites::Entity::delete_many()
            .filter(favorites::Column::RecipeId.eq(recipe_id))
            .filter(favorites::Column::UserId.eq(user_id))
            .exec(db)
            .await?;
        Ok(!(res.rows_affected > 0))
    } else {
        favorites::ActiveModel {
            user_id: Set(user_id),
            recipe_id: Set(recipe_id),
            ..Default::default()
        }
        .insert(db)
        .await?;
        Ok(true)
    }
}
pub async fn rate(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Uuid,
    rating: i32,
) -> Result<(), Error> {
    let existing_rating = recipe_ratings::Entity::find()
        .filter(recipe_ratings::Column::RecipeId.eq(recipe_id))
        .filter(recipe_ratings::Column::UserId.eq(user_id))
        .one(db)
        .await?;
    if let Some(model) = existing_rating {
        let mut active: recipe_ratings::ActiveModel = model.into();
        active.rating = Set(rating);
        active.insert(db).await?;
    } else {
        recipe_ratings::ActiveModel {
            recipe_id: Set(recipe_id),
            user_id: Set(user_id),
            rating: Set(rating),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }
    Ok(())
}
pub async fn unrate(db: &DatabaseConnection, recipe_id: Uuid, user_id: Uuid) -> Result<(), Error> {
    let res = recipe_ratings::Entity::delete_many()
        .filter(recipe_ratings::Column::RecipeId.eq(recipe_id))
        .filter(recipe_ratings::Column::UserId.eq(user_id))
        .exec(db)
        .await?;
    Ok(())
}
pub async fn get_rating(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<RecipeRatingDto, Error> {
    #[derive(FromQueryResult)]
    struct Aggregates {
        avg_rating: Option<f64>,
        count: i64,
    }

    let stats = recipe_ratings::Entity::find()
        .select_only()
        .column_as(recipe_ratings::Column::Rating.avg(), "avg_rating")
        .column_as(recipe_ratings::Column::Rating.count(), "count")
        .filter(recipe_ratings::Column::RecipeId.eq(recipe_id))
        .into_model::<Aggregates>()
        .one(db)
        .await?
        .unwrap_or(Aggregates { avg_rating: None, count: 0 });

    let mut user_rating = None;
    if let Some(uid) = user_id {
        let personal_rating = recipe_ratings::Entity::find_by_id((recipe_id, uid))
            .one(db)
            .await?;

        if let Some(m) = personal_rating {
            user_rating = Some(m.rating);
        }
    }

    Ok(RecipeRatingDto {
        average: stats.avg_rating.unwrap_or(0.0) as f32,
        count: stats.count,
        user_rating,
    })
}
pub async fn get_comment(
    db: &DatabaseConnection,
    comment_id: Uuid,
) -> Result<CommentDto, Error> {
    let res = recipe_comments::Entity::find_by_id(comment_id)
        .find_also_related(users::Entity)
        .one(db)
        .await?;

    if let Some((comment, user_opt)) = res {
        let username = user_opt
            .map(|u| u.username)
            .unwrap_or_else(|| "Deleted User".to_string());

        Ok(CommentDto {
            id: comment.id,
            recipe_id: comment.recipe_id,
            user_id: comment.user_id.unwrap_or_else(Uuid::nil),
            username,
            parent_id: comment.parent_id,
            content: if comment.deleted_at.is_some() {
                "This comment has been deleted.".to_string()
            } else {
                comment.content
            },
            created_at: comment.created_at.with_timezone(&Utc),
            edited_at: comment.edited_at.map(|dt| dt.with_timezone(&Utc)),
            children: Vec::new(),
            deleted_at: comment.deleted_at.map(|dt| dt.with_timezone(&Utc)),
        })
    } else {
        Err(Error::NotFound(serde_json::json!({
            "error": "Comment not found",
            "id": comment_id
        })))
    }
}
pub async fn get_comments(
    db: &DatabaseConnection,
    recipe_id: Uuid,
) -> Result<Vec<CommentDto>, Error> {
    let results = recipe_comments::Entity::find()
        .filter(recipe_comments::Column::RecipeId.eq(recipe_id))
        .find_also_related(users::Entity) // This joins the users table
        .order_by_asc(recipe_comments::Column::CreatedAt)
        .all(db)
        .await?;

    let all_comments: Vec<CommentDto> = results.into_iter().map(|(comment, user_opt)| {
        let username = user_opt
            .map(|u| u.username)
            .unwrap_or_else(|| "Deleted User".to_string());

        CommentDto {
            id: comment.id,
            recipe_id: comment.recipe_id,
            user_id: comment.user_id.unwrap_or_else(Uuid::nil),
            username,
            parent_id: comment.parent_id,
            content: if comment.deleted_at.is_some() {
                "This comment has been deleted.".to_string()
            } else {
                comment.content
            },
            created_at: comment.created_at.with_timezone(&Utc),
            edited_at: Default::default(),
            children: Vec::new(),
            deleted_at: None,
        }
    }).collect();

    let mut children_map: HashMap<Uuid, Vec<CommentDto>> = HashMap::new();

    let mut root_comments = Vec::new();

    for comment in all_comments {
        if let Some(p_id) = comment.parent_id {
            children_map.entry(p_id).or_default().push(comment);
        } else {
            root_comments.push(comment);
        }
    }

    fn nest_replies(parent: &mut CommentDto, map: &mut HashMap<Uuid, Vec<CommentDto>>) {
        if let Some(children) = map.remove(&parent.id) {
            parent.children = children;
            for reply in &mut parent.children {
                nest_replies(reply, map);
            }
        }
    }

    for root in &mut root_comments {
        nest_replies(root, &mut children_map);
    }

    Ok(root_comments)
}
pub async fn add_comment(
    db: &DatabaseConnection,
    new_comment: CreateCommentDto,
    recipe_id: Uuid,
    user_id: Uuid,
) -> Result<CommentDto, Error> {
    let res = recipe_comments::ActiveModel {
        id: Set(Uuid::new_v4()),
        recipe_id: Set(recipe_id),
        user_id: Set(Some(user_id)),
        parent_id: Set(new_comment.parent_id),
        content: Set(new_comment.content),
        created_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    }
        .insert(db)
        .await?;

    let user = users::Entity::find_by_id(user_id).one(db).await?;

    let username = user
        .map(|u| u.username)
        .unwrap_or_else(|| "Unknown User".to_string());

    Ok(CommentDto {
        id: res.id,
        recipe_id: res.recipe_id,
        user_id: res.user_id.unwrap_or(user_id),
        username,
        parent_id: res.parent_id,
        content: res.content,
        created_at: res.created_at.with_timezone(&Utc),
        edited_at: res.edited_at.map(|dt| dt.with_timezone(&Utc)),
        children: Vec::new(),
        deleted_at: res.deleted_at.map(|dt| dt.with_timezone(&Utc)),
    })
}
pub async fn delete_comment(
    db: &DatabaseConnection,
    comment_id: Uuid,
) -> Result<CommentDto, Error> {
    let res = recipe_comments::Entity::find_by_id(comment_id)
        .find_also_related(users::Entity)
        .one(db)
        .await?;

    if let Some((model, user_opt)) = res {
        let mut active: recipe_comments::ActiveModel = model.clone().into();
        active.deleted_at = Set(Some(Utc::now().into()));

        let updated_model = active.update(db).await?;

        let username = user_opt
            .map(|u| u.username)
            .unwrap_or_else(|| "Deleted User".to_string());

        Ok(CommentDto {
            id: updated_model.id,
            recipe_id: updated_model.recipe_id,
            user_id: updated_model.user_id.unwrap_or_else(Uuid::nil),
            username,
            parent_id: updated_model.parent_id,
            content: "".to_string(),
            created_at: updated_model.created_at.with_timezone(&Utc),
            edited_at: updated_model.edited_at.map(|dt| dt.with_timezone(&Utc)),
            children: Vec::new(),
            deleted_at: updated_model.deleted_at.map(|dt| dt.with_timezone(&Utc)),
        })
    } else {
        Err(Error::NotFound(json!({
            "error": "Comment not found",
            "id": comment_id
        })))
    }
}
pub async fn edit_comment(
    db: &DatabaseConnection,
    comment_id: Uuid,
    new_comment: CommentDto,
) -> Result<CommentDto, Error> {
    let res = recipe_comments::Entity::find_by_id(comment_id)
        .find_also_related(users::Entity)
        .one(db)
        .await?;

    if let Some((model, user_opt)) = res {
        let mut active: recipe_comments::ActiveModel = model.into();
        active.content = Set(new_comment.content);

        active.edited_at = Set(Some(chrono::Utc::now().into()));

        let updated_model = active.update(db).await?;

        let username = user_opt
            .map(|u| u.username)
            .unwrap_or_else(|| "Deleted User".to_string());

        Ok(CommentDto {
            id: updated_model.id,
            recipe_id: updated_model.recipe_id,
            user_id: updated_model.user_id.unwrap_or_else(Uuid::nil),
            username,
            parent_id: updated_model.parent_id,
            content: updated_model.content,
            created_at: updated_model.created_at.with_timezone(&Utc),
            edited_at: updated_model.deleted_at.map(|dt| dt.with_timezone(&Utc)),
            children: Vec::new(),
            deleted_at: updated_model.edited_at.map(|dt| dt.with_timezone(&Utc)),
        })
    } else {
        Err(Error::NotFound(json!({
            "error": "Comment not found",
            "id": comment_id
        })))
    }
}
