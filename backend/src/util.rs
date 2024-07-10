use std::{env, process::Command};

use diesel::{Connection, SqliteConnection};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::models::user::User;

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
    info!("Deleting all users");
    info!(
        "{:?}",
        Command::new("diesel").args(["migration", "redo"]).output()
    );
    info!("Loading test users");
    let _ = User::create_hash(true, "Admin", "Adminname", "admin", "admin");
    let _ = User::create_hash(false, "User", "Username", "user", "user");
    info!("Test users loaded {:#?}", User::all());
}
