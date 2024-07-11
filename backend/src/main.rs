pub mod auth;
pub mod models;
pub mod routes;
pub mod schema;
pub mod util;

use crate::routes::login::post_login;
use crate::routes::login::post_register;
use dotenvy::dotenv;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
use routes::login::{okapi_add_operation_for_post_login_, okapi_add_operation_for_post_register_};
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
    info!("Running in debug mode");
    load_test_data();
    #[cfg(not(debug_assertions))]

    info!("Starting rocket");
    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true)
        .to_cors()
        .unwrap();

    rocket::build()
        .attach(cors)
        .mount("/", openapi_get_routes![post_login, post_register])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
