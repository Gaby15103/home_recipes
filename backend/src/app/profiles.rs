use actix_web::{HttpServer, HttpResponse, web::Path, web::Data};
use futures::Future;

use super::AppState;
use crate::prelude::*;
use crate::utils::auth::{authenticate, Auth};