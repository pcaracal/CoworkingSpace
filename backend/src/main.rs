pub mod auth;
pub mod models;
pub mod routes;
pub mod schema;
pub mod util;

use crate::routes::login::get_login;
use crate::routes::rooms::get_rooms;
use crate::routes::users::delete_users;
use crate::routes::users::get_users;
use crate::routes::users::post_users;
use crate::routes::users::put_users;
use crate::routes::{
    bookings::{delete_bookings, get_bookings, patch_bookings, post_bookings},
    login::{post_login, post_register},
};
use dotenvy::dotenv;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
use routes::login::okapi_add_operation_for_get_login_;
use routes::login::{okapi_add_operation_for_post_login_, okapi_add_operation_for_post_register_};
use routes::rooms::okapi_add_operation_for_get_rooms_;
use routes::users::okapi_add_operation_for_delete_users_;
use routes::users::okapi_add_operation_for_post_users_;
use routes::users::okapi_add_operation_for_put_users_;
use routes::{
    bookings::{
        okapi_add_operation_for_delete_bookings_, okapi_add_operation_for_get_bookings_,
        okapi_add_operation_for_patch_bookings_, okapi_add_operation_for_post_bookings_,
    },
    users::okapi_add_operation_for_get_users_,
};
use util::{load_test_data, setup_logger};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

#[launch]
fn rocket() -> _ {
    setup_logger();
    dotenv().ok();

    #[cfg(debug_assertions)]
    load_test_data();
    // #[cfg(not(debug_assertions))]

    info!("Starting rocket");
    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true)
        .to_cors()
        .unwrap();

    rocket::build()
        .attach(cors)
        .mount(
            "/",
            openapi_get_routes![
                post_login,
                post_register,
                get_login,
                get_bookings,
                post_bookings,
                delete_bookings,
                patch_bookings,
                get_users,
                post_users,
                put_users,
                delete_users,
                get_rooms,
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
