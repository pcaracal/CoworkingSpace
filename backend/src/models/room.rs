use diesel::{prelude::*, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket_okapi::{okapi::schemars, JsonSchema};
use serde::{Deserialize, Serialize};

use crate::{schema::room, util::conn};

#[derive(
    JsonSchema,
    Queryable,
    PartialEq,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    Debug,
    AsChangeset,
)]
#[diesel(table_name = crate::schema::room)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Room {
    pub id: Option<i32>,
    pub name: String,
}

impl Room {
    #[must_use]
    pub fn create(name: &str) -> Option<Room> {
        let new_room = Room {
            id: None,
            name: name.to_string(),
        };

        diesel::insert_into(room::table)
            .values(&new_room)
            .execute(&mut conn())
            .ok()?;

        room::table
            .filter(room::name.eq(&name))
            .order(room::id.desc())
            .first(&mut conn())
            .ok()
    }

    #[must_use]
    pub fn all() -> Vec<Room> {
        room::table.load(&mut conn()).unwrap_or_default()
    }

    #[must_use]
    pub fn by_id(id: i32) -> Option<Room> {
        room::table.filter(room::id.eq(id)).first(&mut conn()).ok()
    }

    #[must_use]
    pub fn delete(id: i32) -> bool {
        diesel::delete(room::table.filter(room::id.eq(id)))
            .execute(&mut conn())
            .is_ok()
    }
}
