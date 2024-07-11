use std::env;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_http::Status;
use rocket_okapi::{gen::OpenApiGenerator, okapi::schemars, request::RequestHeaderInput};
use rocket_okapi::{okapi::schemars::JsonSchema, request::OpenApiFromRequest};
use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[allow(clippy::missing_panics_doc)]
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(password_hash).unwrap();
    argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok()
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct Token<'r>(pub &'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(token: &str) -> bool {
            decode_token(token).is_some()
        }

        match request.headers().get_one("Authorization") {
            Some(key) if is_valid(key) => rocket::outcome::Outcome::Success(Token(key)),
            Some(_) | None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for Token<'r> {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[must_use]
pub fn encode_token(id: i32) -> Option<String> {
    let claims = Claims {
        sub: id.to_string(),
        exp: usize::try_from((Utc::now() + chrono::Duration::hours(24)).timestamp()).ok()?,
    };

    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    let jwt_secret = env::var("JWT_SECRET").ok()?;
    let key = EncodingKey::from_secret(jwt_secret.as_ref());

    jsonwebtoken::encode(&header, &claims, &key).ok()
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn decode_token(token: &str) -> Option<i32> {
    if token.is_empty() || token.len() < 7 {
        return None;
    }

    let token = if token.starts_with("Bearer") {
        token[7..].to_string()
    } else {
        token.to_string()
    };

    let jwt_secret = env::var("JWT_SECRET").expect("jwt_secret must be set");
    let key = DecodingKey::from_secret(jwt_secret.as_ref());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    let Ok(token_data) = jsonwebtoken::decode::<Claims>(&token, &key, &validation) else {
        return None;
    };

    token_data.claims.sub.parse::<i32>().ok()
}

#[must_use]
pub fn user_from_token(token: &str) -> Option<User> {
    let id = decode_token(token)?;
    User::by_id(id)
}
