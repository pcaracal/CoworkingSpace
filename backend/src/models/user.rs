use diesel::{prelude::*, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::hash_password,
    schema::{
        self,
        user::{self},
    },
    util::conn,
};

#[allow(clippy::module_name_repetitions)]
#[derive(JsonSchema, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = crate::schema::user)]
pub struct PostUser {
    pub is_admin: bool,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[allow(clippy::module_name_repetitions)]
#[derive(JsonSchema, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = crate::schema::user)]
pub struct PutUser {
    pub is_admin: bool,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(
    Queryable,
    PartialEq,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    Debug,
    AsChangeset,
    JsonSchema,
)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<i32>,
    pub is_admin: Option<bool>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<String>,
}

#[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
impl User {
    #[must_use]
    pub fn create_hash(
        is_admin: bool,
        first_name: &str,
        last_name: &str,
        email: &str,
        password: &str,
    ) -> Option<User> {
        User::create(
            is_admin,
            first_name,
            last_name,
            email,
            &hash_password(password),
        )
    }

    #[must_use]
    pub fn create(
        is_admin: bool,
        first_name: &str,
        last_name: &str,
        email: &str,
        password: &str,
    ) -> Option<User> {
        let new_user = User {
            id: None,
            is_admin: Some(is_admin),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            created_at: None,
        };

        diesel::insert_into(schema::user::table)
            .values(&new_user)
            .execute(&mut conn())
            .ok()?;

        crate::schema::user::table
            .filter(user::email.eq(email))
            .first(&mut conn())
            .ok()
    }

    #[must_use]
    pub fn all() -> Vec<User> {
        schema::user::table.load(&mut conn()).unwrap_or_default()
    }

    #[must_use]
    pub fn by_id(id: i32) -> Option<User> {
        schema::user::table
            .filter(user::id.eq(id))
            .first(&mut conn())
            .ok()
    }

    #[must_use]
    pub fn by_email(email: &str) -> Option<User> {
        schema::user::table
            .filter(user::email.eq(email))
            .first(&mut conn())
            .ok()
    }

    #[must_use]
    pub fn count() -> Option<i64> {
        schema::user::table.count().get_result(&mut conn()).ok()
    }

    #[must_use]
    pub fn update(id: i32, put_user: &PutUser) -> Option<User> {
        diesel::update(schema::user::table.filter(user::id.eq(id)))
            .set(put_user)
            .execute(&mut conn())
            .ok()?;

        schema::user::table
            .filter(user::id.eq(id))
            .first(&mut conn())
            .ok()
    }

    #[must_use]
    pub fn delete(id: i32) -> bool {
        diesel::delete(schema::user::table.filter(user::id.eq(id)))
            .execute(&mut conn())
            .is_ok()
    }
}
