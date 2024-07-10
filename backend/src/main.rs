pub mod auth;
pub mod models;
pub mod schema;
pub mod util;

use dotenvy::dotenv;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
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

    rocket::build().attach(cors).mount("/", routes![])
}
