use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Could not create the Db directory")]
    CreateDbDirFailure(std::io::Error),
    #[error("Could not open the db config file")]
    FailToOpenConfigFile(std::io::Error),
    #[error("Could not deserialize the config file")]
    ConfigDeserError(serde_json::Error),
    #[error("Io error")]
    IoError(std::io::Error),
    #[error("Serialize error")]
    SerializeError(serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerializeError(value)
    }
}
