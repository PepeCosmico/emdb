use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

use super::error::Result;

const CONFIG_FILE: &str = "/config.json";

#[derive(Serialize, Deserialize)]
pub struct Db {
    path: String,
    tables: Vec<String>,
}

impl Db {
    pub fn init(path: &str) -> Result<Db> {
        // Creates the db directory if it wasn't created yet
        std::fs::create_dir_all(path)?;

        // Read or create config file
        if Path::exists(&Path::new(&(path.to_string() + CONFIG_FILE))) {
            let file = File::open(&(path.to_string() + CONFIG_FILE))?;
            let db = serde_json::from_reader(file)?;
            Ok(db)
        } else {
            let db = Db {
                path: path.to_string() + CONFIG_FILE,
                tables: Vec::new(),
            };
            let file = File::create(&(path.to_string() + CONFIG_FILE))?;
            serde_json::to_writer(file, &db)?;
            Ok(db)
        }
    }
}

#[cfg(test)]
#[path = "../../_tests/database/db.rs"]
mod tests;
