use rocket::serde::json::Json;
use rocket_http::Status;
use rocket_okapi::openapi;

use crate::{
    auth::{self, Token},
    models::room::RoomResponse,
};

#[allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]
#[openapi(tag = "Rooms")]
#[get("/rooms")]
/// Only accessible by authenticated users
pub fn get_rooms(token: Token) -> Result<Json<Vec<RoomResponse>>, Status> {
    let Some(user) = auth::user_from_token(token.0) else {
        return Err(Status::Unauthorized);
    };
    info!("GET /rooms called by user: {user:?}");

    let res = RoomResponse::new();

    Ok(Json(res))
}
