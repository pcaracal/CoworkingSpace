use std::borrow::Borrow;

use diesel::query_builder::AsChangeset;
use rocket::serde::json::Json;
use rocket_http::Status;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{self, Token},
    models::{
        booking::{Booking, SerializeBooking, UpdateBooking},
        room::Room,
    },
};

#[allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]
#[openapi(tag = "Bookings")]
#[get("/bookings")]
/// Only accessible by authenticated users
///
/// Users can only see their own bookings
///
/// Admins can see all bookings
pub fn get_bookings(token: Token) -> Result<Json<Vec<SerializeBooking>>, Status> {
    let Some(user) = auth::user_from_token(token.0) else {
        return Err(Status::Unauthorized);
    };

    info!("GET /bookings called by user: {user:?}");

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

#[allow(clippy::module_name_repetitions)]
#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct PostBooking {
    pub reason: String,
    /// 0 = Morning, 1 = Afternoon, 2 = Day
    pub duration: i32,
    /// YYYY-MM-DD
    pub date: String,
    pub room_id: i32,
}

#[allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]
#[openapi(tag = "Bookings")]
#[post("/bookings", data = "<post_booking>")]
/// Only accessible by authenticated users
pub fn post_bookings(
    token: Token,
    post_booking: Json<PostBooking>,
) -> Result<Json<SerializeBooking>, Status> {
    let Some(user) = auth::user_from_token(token.0) else {
        return Err(Status::Unauthorized);
    };
    info!("POST /bookings {post_booking:?} called by user: {user:?}");

    if Room::by_id(post_booking.room_id).is_none() {
        // room not found
        return Err(Status::NotFound);
    };

    let date_bookings = Booking::by_date(&post_booking.date);
    for b in date_bookings {
        if b.fk_room_id == post_booking.room_id
            && (b.duration == post_booking.duration
                || b.duration == 2
                || post_booking.duration == 2)
        {
            // if duration is same || old booking is day || new booking is day
            return Err(Status::Conflict);
        }
    }

    if let Some(nb) = Booking::create(
        &post_booking.reason,
        post_booking.duration,
        &post_booking.date,
        post_booking.room_id,
        user.id.unwrap_or_default(),
    ) {
        match SerializeBooking::from_booking(nb) {
            Some(b) => Ok(Json(b)),
            None => Err(Status::InternalServerError),
        }
    } else {
        Err(Status::InternalServerError)
    }
}

#[allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]
#[openapi(tag = "Bookings")]
#[delete("/bookings/<id>")]
/// Users can only delete their own bookings
///
/// Admins can delete any booking
pub fn delete_bookings(id: i32, token: Token) -> Result<Status, Status> {
    let Some(user) = auth::user_from_token(token.0) else {
        return Err(Status::Unauthorized);
    };
    info!("DELETE /bookings/{id:?} called by user: {user:?}");

    let Some(booking) = Booking::by_id(id) else {
        return Ok(Status::NoContent);
    };
    let is_admin = user.is_admin.unwrap_or_default();

    if booking.fk_user_id == user.id.unwrap_or_default() || is_admin {
        if Booking::delete(id) {
            Ok(Status::NoContent)
        } else {
            Err(Status::InternalServerError)
        }
    } else {
        Err(Status::Forbidden)
    }
}

#[allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]
#[openapi(tag = "Bookings")]
#[patch("/bookings/<id>", data = "<update_booking>")]
/// Only accessible by admins
///
/// Set reason, date, status to empty string to leave unchanged
///
/// Set duration to -1 to leave unchanged
pub fn patch_bookings(
    id: i32,
    token: Token,
    update_booking: Json<UpdateBooking>,
) -> Result<Json<SerializeBooking>, Status> {
    let ub = update_booking.0;
    let mut update_booking = UpdateBooking {
        reason: None,
        duration: None,
        status: None,
        date: None,
    };

    if ub.reason.clone().is_some_and(|x| !x.is_empty()) {
        update_booking.reason = Some(ub.reason.unwrap());
    }
    if ub.duration.is_some() && ub.duration.unwrap() > -1 && ub.duration.unwrap() < 3 {
        update_booking.duration = Some(ub.duration.unwrap());
    }
    if ub.status.clone().is_some_and(|x| !x.is_empty()) {
        update_booking.status = Some(ub.status.unwrap());
    }
    if ub.date.clone().is_some_and(|x| !x.is_empty()) {
        update_booking.date = Some(ub.date.unwrap());
    }

    let Some(user) = auth::user_from_token(token.0) else {
        return Err(Status::Unauthorized);
    };
    info!("PATCH /bookings/{id:?} called by user: {user:?}");

    if !user.is_admin.unwrap_or_default() {
        return Err(Status::Forbidden);
    }

    if Booking::by_id(id).is_none() {
        return Err(Status::NotFound);
    };

    let updated = Booking::update(id, update_booking);

    match updated {
        Some(nb) => match SerializeBooking::from_booking(nb) {
            Some(sb) => Ok(Json(sb)),
            None => Err(Status::InternalServerError),
        },
        None => Err(Status::InternalServerError),
    }
}
