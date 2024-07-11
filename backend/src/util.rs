use std::{env, process::Command};

use diesel::{Connection, SqliteConnection};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::models::{room::Room, user::User};

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

pub fn load_test_data() {
    warn!("Deleting all data");
    info!(
        "{:?}",
        Command::new("diesel").args(["migration", "redo"]).output()
    );
    info!("Loading test users");
    info!(
        "{:?}",
        User::create_hash(true, "Admin", "Adminname", "admin", "admin")
    );
    info!(
        "{:?}",
        User::create_hash(false, "User", "Username", "user", "user")
    );
    warn!("Test users loaded");

    warn!("Loading test rooms");
    info!("{:?}", Room::create("Room 1"));
    info!("{:?}", Room::create("Room 2"));
    info!("{:?}", Room::create("Room 3"));
    info!("{:?}", Room::create("Room 4"));
    warn!("Test rooms loaded");
}
