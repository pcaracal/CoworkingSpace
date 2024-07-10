use dotenvy::dotenv;
use log::LevelFilter;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

fn setup_logger() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Always,
    )
    .unwrap();
}

#[launch]
fn rocket() -> _ {
    setup_logger();
    dotenv().ok();

    info!("Starting rocket");
    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true)
        .to_cors()
        .unwrap();

    rocket::build().attach(cors).mount("/", routes![])
}
