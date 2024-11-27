use homedir;
use rusqlite;
use std::io;

#[derive(thiserror::Error, Debug)]
pub enum ProtonCtlError {
    #[error("Filesystem operation failed")]
    Io(#[from] io::Error),
    #[error("SQL operation failed")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Failed to locate home")]
    HomeDir(#[from] homedir::GetHomeError),
    #[error("Custom error: {0}")]
    Custom(String),
}
