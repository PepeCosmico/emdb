use std::fs::File;

use super::{db::CONFIG_FILE, error::Result, Db, Error};

impl Db {
    pub fn load_config(&mut self) -> Result<()> {
        let file = File::open(&(self.path.to_string() + CONFIG_FILE))
            .map_err(|err| Error::FailToOpenConfigFile(err))?;
        let db = serde_json::from_reader::<File, Db>(file)
            .map_err(|err| Error::ConfigDeserError(err))?;
        self.tables = db.tables;
        Ok(())
    }
}
