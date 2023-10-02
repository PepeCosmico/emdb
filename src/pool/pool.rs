use crate::database::Db;

use super::error::Result;

const DB_DEFAULT_PATH: &str = "./db";

pub struct Pool {
    db: Db,
}

impl Pool {
    pub fn init_to_path(path: &str) -> Result<Self> {
        Ok(Pool {
            db: Db::init(path)?,
        })
    }

    pub fn init() -> Result<Self> {
        Self::init_to_path(DB_DEFAULT_PATH)
    }
}

#[cfg(test)]
#[path = "../../_tests/pool/pool.rs"]
mod tests;
