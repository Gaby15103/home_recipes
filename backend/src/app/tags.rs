use actix_web::{
    HttpRequest, HttpResponse,
    web::{Data, Json},
};
use validator::Validate;

use super::AppState;
use crate::models::Tag;
use crate::prelude::*;
use crate::utils::auth::{Auth, authenticate};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct In<U> {
    tag: U,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateTag {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputTag {
    Existing { id: Uuid },
    New { name: String },
}

#[derive(Debug)]
pub struct CreateTagOuter {
    pub auth: Auth,
    pub new_tag: CreateTag,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateTag {
    pub id: Uuid,
    #[validate(length(min = 1, max = 32))]
    pub name: String,
}
#[derive(Debug)]
pub struct UpdateTagOuter {
    pub auth: Auth,
    pub update_tag: UpdateTag,
}

#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub tag: TagResponseInner,
}

#[derive(Debug, Serialize)]
pub struct TagResponseInner {
    pub id: Uuid,
    pub name: String,
}

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        TagResponse {
            tag: TagResponseInner {
                id: tag.id,
                name: tag.name,
            },
        }
    }
}

pub async fn create(
    state: Data<AppState>,
    (form, req): (Json<In<CreateTag>>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let new_tag = form.into_inner().tag;

    new_tag.validate()?;

    let auth = authenticate(&state, &req).await?;

    let res = state
        .db
        .send(CreateTagOuter { auth, new_tag })
        .await
        .map_err(|_| crate::error::Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn update(
    state: Data<AppState>,
    (form, req): (Json<In<UpdateTag>>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let update_tag = form.into_inner().tag;

    update_tag.validate()?;

    let auth = authenticate(&state, &req).await?;

    let res = state
        .db
        .send(UpdateTagOuter { auth, update_tag })
        .await
        .map_err(|_| Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
}
pub struct GetAllTags;
pub async fn get_all(
    state: Data<AppState>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let tags = state
        .db
        .send(GetAllTags{})
        .await
        .map_err(|_| Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(tags))
}
