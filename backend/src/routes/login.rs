use std::borrow::Borrow;

use rocket::{response::status, serde::json::Json};
use rocket_http::Status;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use rocket_okapi::{okapi::schemars, OpenApiFromRequest};
use serde::{Deserialize, Serialize};

use crate::{auth, models::user::User};

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[allow(clippy::module_name_repetitions)]
#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

#[openapi(tag = "Login")]
#[post("/login", data = "<login>")]
#[allow(clippy::missing_panics_doc)]
pub fn post_login(login: Json<Login>) -> Result<Json<LoginResponse>, Status> {
    info!("Login attempt: {:?}", login.email);

    match User::by_email(&login.email) {
        Some(u) => {
            if auth::verify_password(&login.password, &u.password) {
                match auth::encode_token(u.id.unwrap()) {
                    Some(token) => {
                        let response = LoginResponse {
                            user: u,
                            token: token.clone(),
                        };

                        info!("User logged in: {response:?}");

                        return Ok(Json(response));
                    }
                    None => return Err(Status::InternalServerError),
                }
            }

            Err(Status::Unauthorized)
        }
        _ => Err(Status::Unauthorized),
    }
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
