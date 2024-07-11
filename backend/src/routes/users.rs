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

#[allow(clippy::module_name_repetitions)]
#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct PostUser {
    pub is_admin: bool,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]
#[openapi(tag = "Users")]
#[post("/users", data = "<post_user>")]
/// Only accessible by admins
pub fn post_users(post_user: Json<PostUser>, token: Token) -> Result<Json<User>, Status> {
    let Some(user) = auth::user_from_token(token.0) else {
        return Err(Status::Unauthorized);
    };
    info!("POST /users called by user: {user:?}");

    if !user.is_admin.unwrap_or_default() {
        return Err(Status::Unauthorized);
    }

    if User::by_email(&post_user.email).is_some() {
        return Err(Status::Conflict);
    }

    let new_user = User::create_hash(
        post_user.is_admin,
        &post_user.first_name,
        &post_user.last_name,
        &post_user.email,
        &post_user.password,
    );

    match new_user {
        Some(u) => Ok(Json(u)),
        None => Err(Status::InternalServerError),
    }
}
