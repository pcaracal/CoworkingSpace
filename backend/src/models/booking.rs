use diesel::{prelude::*, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{schema::booking, util::conn};
use rocket_okapi::okapi::schemars;

use super::{room::Room, user::User};

#[derive(
    Queryable, PartialEq, Selectable, Insertable, Serialize, Deserialize, Debug, AsChangeset,
)]
#[diesel(table_name = crate::schema::booking)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Booking {
    pub id: Option<i32>,
    pub reason: String,
    /// 0 = Morning, 1 = Afternoon, 2 = Day
    pub duration: i32,
    pub status: String,
    /// YYYY-MM-DD
    pub date: String,
    pub fk_room_id: i32,
    pub fk_user_id: i32,
    pub created_at: Option<String>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct SerializeBooking {
    pub id: Option<i32>,
    pub reason: String,
    /// 0 = Morning, 1 = Afternoon, 2 = Day
    pub duration: i32,
    pub status: String,
    /// YYYY-MM-DD
    pub date: String,
    pub room: Room,
    pub user: User,
    pub created_at: Option<String>,
}

impl SerializeBooking {
    #[must_use]
    pub fn from_booking(booking: Booking) -> Option<Self> {
        let room = Room::by_id(booking.fk_room_id)?;
        let user = User::by_id(booking.fk_user_id)?;

        Some(Self {
            id: booking.id,
            reason: booking.reason,
            duration: booking.duration,
            status: booking.status,
            date: booking.date,
            room,
            user,
            created_at: booking.created_at,
        })
    }
}

impl Booking {
    #[must_use]
    pub fn create(
        reason: &str,
        duration: i32,
        date: &str,
        fk_room_id: i32,
        fk_user_id: i32,
    ) -> Option<Booking> {
        let new_booking = Booking {
            id: None,
            reason: reason.to_string(),
            duration,
            status: "Pending".to_string(),
            date: date.to_string(),
            fk_room_id,
            fk_user_id,
            created_at: None,
        };

        diesel::insert_into(booking::table)
            .values(&new_booking)
            .execute(&mut conn())
            .ok()?;

        booking::table
            .order(booking::id.desc())
            .first(&mut conn())
            .ok()
    }

    #[must_use]
    pub fn all() -> Vec<Booking> {
        booking::table.load(&mut conn()).unwrap_or_default()
    }

    #[must_use]
    pub fn by_id(id: i32) -> Option<Booking> {
        booking::table
            .filter(booking::id.eq(id))
            .first(&mut conn())
            .ok()
    }

    #[must_use]
    pub fn by_user_id(id: i32) -> Vec<Booking> {
        booking::table
            .filter(booking::fk_user_id.eq(id))
            .load(&mut conn())
            .unwrap_or_default()
    }

    #[must_use]
    pub fn by_date(date: &str) -> Vec<Booking> {
        booking::table
            .filter(booking::date.eq(date))
            .load(&mut conn())
            .unwrap_or_default()
    }

    #[must_use]
    pub fn update_status(id: i32, status: &str) -> Option<Booking> {
        diesel::update(booking::table.filter(booking::id.eq(id)))
            .set(booking::status.eq(status))
            .execute(&mut conn())
            .ok()?;

        booking::table
            .filter(booking::id.eq(id))
            .first(&mut conn())
            .ok()
    }

    #[must_use]
    pub fn delete(id: i32) -> bool {
        diesel::delete(booking::table.filter(booking::id.eq(id)))
            .execute(&mut conn())
            .is_ok()
    }
}
