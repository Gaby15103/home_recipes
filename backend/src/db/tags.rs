use actix::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;
use super::DbExecutor;
use crate::app::tags::{GetAllTags};
use crate::dto::{CreateTagOuter, InputTag, TagResponse, UpdateTagOuter};
use crate::models::{NewTag, Tag, TagChange};
use crate::prelude::*;
use crate::schema::{recipe_tags, tags};
use crate::schema::recipes::dsl::recipes;

impl Message for CreateTagOuter {
    type Result = Result<TagResponse>;
}


impl Handler<CreateTagOuter> for DbExecutor {
    type Result = Result<TagResponse>;

    fn handle(&mut self, msg: CreateTagOuter, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::tags::dsl::*;

        let mut conn = self.0.get()?;

        let new_tag = NewTag {
            name: msg.new_tag.name,
        };

        let inserted_tag: Tag = diesel::insert_into(tags)
            .values(&new_tag)
            .get_result(&mut conn)?;

        Ok(TagResponse::from(inserted_tag))
    }
}

/// Resolve or create tags and associate them with a recipe.
///
/// # Arguments
/// * `conn` - Diesel PgConnection
/// * `recipe_id_val` - The recipe to associate tags with
/// * `input_tags` - Vec<InputTag> from the recipe input
pub fn create_or_associate_tags(
    conn: &mut PgConnection,
    recipe_id_val: Uuid,
    input_tags: Vec<InputTag>,
) -> Result<Vec<TagResponse>, diesel::result::Error> {
    use crate::schema::{tags, recipe_tags};

    let mut result_tags = Vec::with_capacity(input_tags.len());

    for input_tag in input_tags {
        // --- Resolve tag (existing or new)
        let resolved_tag: Tag = match input_tag {
            InputTag::Existing { id: existing_id } => {
                tags::table.filter(tags::id.eq(existing_id))
                    .first::<Tag>(conn)?
            }
            InputTag::New { name: tag_name } => {
                let normalized_name = tag_name.trim();
                match tags::table.filter(tags::name.eq(normalized_name)).first::<Tag>(conn) {
                    Ok(tag) => tag, // tag already exists
                    Err(diesel::result::Error::NotFound) => {
                        diesel::insert_into(tags::table)
                            .values(NewTag { name: normalized_name.to_string() })
                            .get_result(conn)?
                    }
                    Err(e) => return Err(e),
                }
            }
        };

        // --- Associate tag with recipe (ignore duplicates)
        diesel::insert_into(recipe_tags::table)
            .values((
                recipe_tags::recipe_id.eq(recipe_id_val),
                recipe_tags::tag_id.eq(resolved_tag.id),
            ))
            .on_conflict((recipe_tags::recipe_id, recipe_tags::tag_id))
            .do_nothing()
            .execute(conn)?;

        result_tags.push(resolved_tag.into());
    }

    Ok(result_tags)
}


pub fn fetch_tags_for_recipe(
    conn: &mut PgConnection,
    recipe_id: Uuid,
) -> Result<Vec<TagResponse>, Error> {
    use crate::schema::{recipe_tags, tags};

    // inner_join is fine; will return empty Vec if no tags exist
    let tags_list: Vec<Tag> = tags::table
        .inner_join(recipe_tags::table.on(recipe_tags::tag_id.eq(tags::id)))
        .filter(recipe_tags::recipe_id.eq(recipe_id))
        .select(tags::all_columns)
        .load(conn)?;

    Ok(tags_list.into_iter().map(TagResponse::from).collect())
}



impl Message for UpdateTagOuter {
    type Result = Result<TagResponse>;
}

impl Handler<UpdateTagOuter> for DbExecutor {
    type Result = Result<TagResponse>;

    fn handle(&mut self, msg: UpdateTagOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tags::dsl::*;

        let mut conn = self.0.get()?;


        let updated_tag = TagChange {
            name: msg.update_tag.name,
        };

        match diesel::update(tags.find(msg.update_tag.id))
            .set(&updated_tag)
            .get_result::<Tag>(&mut conn)
        {
            Ok(tag) => Ok(tag.into()),
            Err(e) => Err(e.into()),
        }
    }
}

impl Message for GetAllTags {
type Result = Result<Vec<TagResponse>>;
}

impl Handler<GetAllTags> for DbExecutor {
    type Result = Result<Vec<TagResponse>>;

    fn handle(&mut self, msg: GetAllTags, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tags::dsl::*;

        let mut conn = self.0.get()?;
        let tag_models: Vec<Tag> = tags
            .order(name.asc())
            .load(&mut conn)?;

        let responses = tag_models
            .into_iter()
            .map(TagResponse::from)
            .collect();

        Ok(responses)
    }
}