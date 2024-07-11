use rocket::serde::json::Json;
use rocket_http::Status;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::{Deserialize, Serialize};

use crate::{auth, models::user::User};

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[openapi(tag = "Login")]
#[post("/login", data = "<login>")]
#[allow(clippy::missing_panics_doc)]
pub fn post_login(login: Json<Login>) -> (Status, Option<String>) {
    let users = User::all();
    for u in users {
        if u.email == login.email && auth::verify_password(&login.password, &u.password) {
            match auth::encode_token(u.id.unwrap()) {
                Some(token) => return (Status::Ok, Some(token)),
                None => return (Status::InternalServerError, None),
            }
        }
    }

    (Status::Unauthorized, None)
}
