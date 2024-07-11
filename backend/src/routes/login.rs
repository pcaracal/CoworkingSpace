use std::borrow::Borrow;

use rocket::{response::status, serde::json::Json};
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
pub fn post_login(login: Json<Login>) -> Result<String, Status> {
    let users = User::all();
    for u in users {
        if u.email == login.email && auth::verify_password(&login.password, &u.password) {
            match auth::encode_token(u.id.unwrap()) {
                Some(token) => return Ok(token),
                None => return Err(Status::InternalServerError),
            }
        }
    }

    Err(Status::Unauthorized)
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct Register {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[openapi(tag = "Login")]
#[post("/register", data = "<register>")]
#[allow(clippy::missing_panics_doc)]
pub fn post_register(register: Json<Register>) -> Result<String, Status> {
    info!("POST /register");

    if User::by_email(&register.email).is_some() {
        info!("Conflict: {register:?}");
        return Err(Status::Conflict);
    }

    let is_admin = match User::count() {
        Some(c) => c == 0,
        None => return Err(Status::InternalServerError),
    };

    match User::create_hash(
        is_admin,
        &register.first_name,
        &register.last_name,
        &register.email,
        &register.password,
    ) {
        Some(user) => match auth::encode_token(user.id.unwrap()) {
            Some(token) => {
                info!("User created: {user:?}");
                Ok(token)
            }
            None => Err(Status::InternalServerError),
        },
        None => Err(Status::InternalServerError),
    }
}
