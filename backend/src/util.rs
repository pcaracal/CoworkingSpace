use std::{
    env::{self, args},
    process::Command,
};

use diesel::{Connection, SqliteConnection};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::models::{
    booking::{Booking, SerializeBooking},
    room::Room,
    user::User,
};

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

pub fn init_data() {
    let args = args().map(|x| x.to_lowercase()).collect::<Vec<String>>();

    for a in args {
        if a.contains("clean") {
            warn!("##################################################");
            warn!("Running with argument clean");
            warn!("Cleaning database");
            info!(
                "{:?}",
                Command::new("diesel").args(["migration", "redo"]).output()
            );
            warn!("Database cleaned");
            warn!("##################################################");
        }

        if a.contains("test") {
            warn!("##################################################");
            warn!("Running with argument test");
            load_test_data();
            warn!("Test data loaded");
            warn!("##################################################");
        }
    }
}

#[allow(clippy::missing_panics_doc)]
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
    warn!("Test rooms loaded");

    warn!("Loading test bookings");
    let b = Booking::create("Reason 1", 0, "2024-07-11", 1, 1);
    info!("{b:?}");
    info!("{:?}", Booking::create("Reason 2", 1, "2024-07-11", 1, 1));
    info!("{:?}", Booking::create("Reason 3", 2, "2024-07-12", 1, 2));
    info!("{:?}", Booking::create("Reason 4", 2, "2024-07-12", 2, 1));
    info!("{:?}", Booking::create("Reason 5", 0, "2024-07-13", 1, 2));
    info!("{:?}", Booking::create("Reason 6", 1, "2024-07-13", 1, 2));
    warn!("Test bookings loaded");

    warn!("Test booking");
    info!("{:#?}", SerializeBooking::from_booking(b.unwrap()));
    warn!("Test booking");
}
