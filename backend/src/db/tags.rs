use actix::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;
use super::DbExecutor;
use crate::app::tags::{CreateTagOuter, UpdateTagOuter};
use crate::dto::{InputTag, TagResponse};
use crate::models::{NewTag, Tag, TagChange};
use crate::prelude::*;
impl Message for CreateTagOuter {
    type Result = Result<TagResponse>;
}


impl Handler<CreateTagOuter> for DbExecutor{
    type Result = Result<TagResponse>;

    fn handle(&mut self, msg: CreateTagOuter, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::tags::dsl::*;

        let mut conn = self.0.get()?;

        let new_tag =  NewTag{
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
    let mut result_tags = Vec::with_capacity(input_tags.len());

    for input_tag in input_tags {
        use crate::schema::tags::dsl::*;
        let resolved_tag: Tag = match input_tag {
            InputTag::Existing { id: existing_id } => {
                // Fetch the existing tag
                tags.filter(id.eq(existing_id))
                    .first::<Tag>(conn)?
            }
            InputTag::New { name: tag_name } => {
                // Normalize or trim if needed
                let normalized_name = tag_name.trim();

                match tags.filter(name.eq(normalized_name)).first::<Tag>(conn) {
                    Ok(tag) => tag, // tag already exists
                    Err(diesel::result::Error::NotFound) => {
                        // Insert new tag
                        diesel::insert_into(tags)
                            .values(NewTag { name: normalized_name.to_string() })
                            .get_result(conn)?
                    }
                    Err(e) => return Err(e),
                }
            }
        };

        // Associate tag with recipe in recipe_tags
        use crate::schema::recipe_tags::dsl::*;
        diesel::insert_into(recipe_tags)
            .values((recipe_id.eq(recipe_id_val), tag_id.eq(resolved_tag.id)))
            .execute(conn)?;

        // Push to result vec
        result_tags.push(resolved_tag.into());
    }

    Ok(result_tags)
}

impl Message for UpdateTagOuter {
    type Result = Result<TagResponse>;
}

impl Handler<UpdateTagOuter> for DbExecutor{
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