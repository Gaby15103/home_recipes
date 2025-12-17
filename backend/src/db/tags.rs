use actix::prelude::*;
use diesel::prelude::*;

use super::DbExecutor;
use crate::app::tags::{CreateTagOuter, TagResponse, UpdateTagOuter};
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