use actix::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;
use super::DbExecutor;
use crate::app::tags::{CreateTagOuter, InputTag, TagResponse, UpdateTagOuter};
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
) -> Result<(), diesel::result::Error> {

    for input_tag in input_tags {
        let resolved_tag_id: Uuid = match input_tag {
            InputTag::Existing { id: other_id } => {
                use crate::schema::tags::dsl::*;
                tags.filter(id.eq(other_id)) // <-- here we rename to avoid shadowing
                    .select(id)
                    .first::<Uuid>(conn)?
            }
            InputTag::New { name: tag_name } => {
                use crate::schema::tags::dsl::*;
                match tags
                    .filter(name.eq(&tag_name))
                    .select(id)
                    .first::<Uuid>(conn)
                {
                    Ok(existing_id) => existing_id,
                    Err(diesel::result::Error::NotFound) => diesel::insert_into(tags)
                        .values(NewTag {
                            name: tag_name.clone(),
                        })
                        .returning(id)
                        .get_result(conn)?,
                    Err(e) => return Err(e.into()),
                }
            }
        };

        use crate::schema::recipe_tags::dsl::*;
        diesel::insert_into(recipe_tags)
            .values((recipe_id.eq(recipe_id_val), tag_id.eq(resolved_tag_id)))
            .execute(conn)?;
    }

    Ok(())
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