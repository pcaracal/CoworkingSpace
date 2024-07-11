use rocket::serde::json::Json;
use rocket_http::Status;
use rocket_okapi::openapi;

use crate::{
    auth::{self, Token},
    models::booking::{Booking, SerializeBooking},
};

#[allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]
#[openapi(tag = "Bookings")]
#[get("/bookings")]
pub fn get_bookings(token: Token) -> Result<Json<Vec<SerializeBooking>>, Status> {
    let Some(user) = auth::user_from_token(token.0) else {
        return Err(Status::Unauthorized);
    };

    let bookings = match user.is_admin {
        Some(true) => Booking::all(),
        Some(false) => Booking::by_user_id(user.id.unwrap_or_default()),
        None => return Err(Status::InternalServerError),
    };

    let mut s_bookings: Vec<SerializeBooking> = Vec::new();

    for b in bookings {
        if let Some(s_booking) = SerializeBooking::from_booking(b) {
            s_bookings.push(s_booking);
        }
    }

    Ok(Json(s_bookings))
}
