use std::borrow::Borrow;

use rocket::{response::status, serde::json::Json};
use rocket_http::Status;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{self, Token},
    models::user::User,
};

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
/// Login with email and password
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
/// Register with first name, last name, email and password.
///
/// First user is automatically an admin
pub fn post_register(register: Json<Register>) -> Result<Json<LoginResponse>, Status> {
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
                let response = LoginResponse {
                    user,
                    token: token.clone(),
                };

                Ok(Json(response))
            }
            None => Err(Status::InternalServerError),
        },
        None => Err(Status::InternalServerError),
    }
}

#[openapi(tag = "Login")]
#[get("/login")]
#[allow(clippy::missing_panics_doc)]
/// Check login status
pub fn get_login(token: Token) -> Result<Json<User>, Status> {
    let Some(user) = auth::user_from_token(token.0) else {
        return Err(Status::Unauthorized);
    };

    info!("GET /login called by user: {user:?}");

    Ok(Json(user))
}
