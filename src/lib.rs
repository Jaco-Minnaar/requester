#[macro_use]
extern crate diesel;

use std::{fs::File, io, path::Path};

use config::ReqstrConfig;
use crossterm::terminal::enable_raw_mode;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use interface::main_window::MainWindow;
use lazy_static::lazy_static;

mod config;
mod interface;
mod models;
mod schema;
mod services;
mod types;

lazy_static! {
    static ref CONFIG: ReqstrConfig = {
        let file_path = "config/rqstr.yaml";

        if let Ok(content) = File::open(file_path) {
            serde_yaml::from_reader(content).expect("Error parsing config file")
        } else {
            ReqstrConfig::default()
        }
    };
}

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(&CONFIG.storage_location)
        .expect(format!("Error connecting to {}", CONFIG.storage_location).as_str())
}

pub fn start() {
    enable_raw_mode().expect("Could not enable raw mode");

    MainWindow::new().run();
}
