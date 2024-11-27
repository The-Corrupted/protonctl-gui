use homedir;
use std::boxed::Box;
use std::path::PathBuf;
use std::sync::OnceLock;

use super::error::ProtonCtlError;

static HOME_DIR: OnceLock<Option<PathBuf>> = OnceLock::new();

pub fn create_directory(path: &PathBuf) -> Result<(), ProtonCtlError> {
    // If the directory we are going to store the database in can't be made, then the rest of the program can't function. We will
    // be unable to store new installs or locate existing installs. This is not a recoverable state.
    if let Err(e) = std::fs::DirBuilder::new().recursive(true).create(path) {
        return Err(ProtonCtlError::from(e));
    }
    Ok(())
}

pub fn get_home_dir() -> Result<&'static PathBuf, ProtonCtlError> {
    HOME_DIR.get_or_init(|| homedir::my_home().unwrap_or(None));

    HOME_DIR
        .get()
        .and_then(|opt| opt.as_ref())
        .ok_or(ProtonCtlError::Custom(
            "Failed to get home directory".to_owned(),
        ))
}

pub fn get_db_path() -> Result<PathBuf, ProtonCtlError> {
    let mut path = get_home_dir()?.clone();
    path.push(".local/share/protonctl");
    create_directory(&path)?;
    path.push("installed.db");
    Ok(path)
}
