use std::env;

use diesel::{Connection, SqliteConnection};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

#[allow(clippy::missing_panics_doc)]
pub fn setup_logger() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Always,
    )
    .unwrap();
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn conn() -> SqliteConnection {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&url).expect("Error connecting to {url}")
}
