use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, Set};
use sea_orm::QueryFilter;
use sea_orm::{EntityTrait, DatabaseConnection};
use serde_json::json;
use uuid::Uuid;
use crate::domain::user::NewUser;
use entity::prelude::{EmailVerificationTokens, Users};
use entity::{users, email_verification_tokens, user_roles, roles};
use entity::users::Model;
use crate::errors::Error;

