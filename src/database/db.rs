use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

use super::error::Result;

use crate::instructions::{data::CreateTable, Instructions};

const DB_DEFAULT_PATH: &str = "./db";
pub const CONFIG_FILE: &str = "/config.json";

#[derive(Serialize, Deserialize)]
pub struct Db {
    pub path: String,
    pub tables: Vec<String>,
}

impl Db {
    pub fn new() -> Db {
        Self::new_with_path(DB_DEFAULT_PATH)
    }

    fn new_with_path(path: &str) -> Db {
        Db {
            path: path.to_string(),
            tables: Vec::new(),
        }
    }

    pub fn init() -> Result<Db> {
        let db = Self::new();

        // Creates the db directory if it wasn't created yet
        std::fs::create_dir_all(db.path).map_err(|err| super::Error::CreateDbDirFailure(err))?;

        // Read or create config file
        if Path::exists(&Path::new(&(db.path.to_string() + CONFIG_FILE))) {
            db.load_config()?;
        } else {
            let file = File::create(&(path.to_string() + CONFIG_FILE))?;
            serde_json::to_writer(file, &db)?;
        }
        Ok(db)
    }

    pub fn exec_query<T: Serialize>(&mut self, query: Instructions<T>) -> Result<()> {
        match query {
            Instructions::Select(data) => Ok(()),
            Instructions::Update(data) => Ok(()),
            Instructions::Delete(data) => Ok(()),
            Instructions::Insert(data) => Ok(()),
            Instructions::CreateTable(data) => self.create_table(data),
        }
    }

    fn create_table(&mut self, create_table: CreateTable) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
#[path = "../../_tests/database/db.rs"]
mod tests;
