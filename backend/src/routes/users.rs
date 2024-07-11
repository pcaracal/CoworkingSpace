use rocket::serde::json::Json;
use rocket_http::Status;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{self, Token},
    models::user::User,
};

#[allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]
#[openapi(tag = "Users")]
#[get("/users")]
/// Only accessible by admins
pub fn get_users(token: Token) -> Result<Json<Vec<User>>, Status> {
    let Some(user) = auth::user_from_token(token.0) else {
        return Err(Status::Unauthorized);
    };

    info!("GET /users called by user: {user:?}");

    if user.is_admin.unwrap_or_default() {
        Ok(Json(User::all()))
    } else {
        Err(Status::Unauthorized)
    }
}
