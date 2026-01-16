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
use crate::dto::{CreateTag, CreateTagOuter, UpdateTag, UpdateTagOuter};

#[derive(Debug, Deserialize)]
pub struct In<U> {
    tag: U,
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
pub async fn list(
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
