use std::env;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqstrConfig {
    pub storage_location: String,
}

impl Default for ReqstrConfig {
    fn default() -> Self {
        let path: String = format!("{}/data/database.sqlite3", env::args().next().unwrap());
        Self {
            storage_location: path,
        }
    }
}
