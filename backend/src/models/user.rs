use diesel::{
    prelude::*, AsChangeset, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl,
    Selectable,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::hash_password,
    schema::{
        self,
        user::{self},
    },
    util::conn,
};

#[derive(
    Queryable, PartialEq, Selectable, Insertable, Serialize, Deserialize, Debug, AsChangeset,
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
    pub created_at: Option<chrono::NaiveDateTime>,
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
    ) -> User {
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
    ) -> User {
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
            .unwrap();

        crate::schema::user::table
            .filter(user::email.eq(email))
            .first(&mut conn())
            .unwrap()
    }

    #[must_use]
    pub fn all() -> Vec<User> {
        schema::user::table.load(&mut conn()).unwrap()
    }

    #[must_use]
    pub fn by_id(id: i32) -> User {
        schema::user::table
            .filter(user::id.eq(id))
            .first(&mut conn())
            .unwrap()
    }
}
