use crate::plib::error::ProtonCtlError;
use crate::plib::fsutils::get_db_path;
use rusqlite;
use std::sync::Arc;

pub enum InstallType {
    WINE,
    UMU,
    PROTON,
    CUSTOM,
}

pub struct InstalledVersion {
    // Name of the install. In most cases this will be the same as the version or asset name
    // We will allow the user to change this
    pub name: String,
    // Location of the install. This can be edited from the homepage
    pub location: String,
    // The type of install. This is meant to be used primarily for filtering.
    pub install_type: InstallType,
}

impl InstalledVersion {
    pub fn new(name: String, location: String, install_type: InstallType) -> Self {
        Self {
            name,
            location,
            install_type,
        }
    }
}

pub struct AppDB {
    db_connection: Arc<rusqlite::Connection>,
}

impl AppDB {
    pub fn new() -> Result<Self, ProtonCtlError> {
        let path = get_db_path()?;
        let result = rusqlite::Connection::open(path);
        if let Err(e) = result {
            return Err(ProtonCtlError::from(e));
        }

        Ok(Self {
            db_connection: Arc::new(result.unwrap()),
        })
    }
}

impl AppDB {
    pub fn create_db_or_do_nothing(&self) -> Result<(), ProtonCtlError> {
        let create_table_sql = r#"CREATE TABLE IF NOT EXISTS installs ( id INTEGER PRIMARY KEY, install_type TEXT NOT NULL, name TEXT NOT NULL UNIQUE, location TEXT NOT NULL);"#;
        if let Err(e) = self.db_connection.execute(create_table_sql, []) {
            return Err(ProtonCtlError::from(e));
        }
        Ok(())
    }

    pub fn get_entries(&self) -> Result<Vec<InstalledVersion>, ProtonCtlError> {
        let get_entries = r#"SELECT name, location, install_type FROM installs"#;
        let mut stmt = self.db_connection.prepare(get_entries)?;
        let entries = stmt
            .query_map([], |row| {
                let mut install_type = InstallType::PROTON;
                let install_type_text: String = row.get(2)?;
                if (install_type_text == "WINE") {
                    install_type = InstallType::WINE
                } else if (install_type_text == "PROTON") {
                    install_type = InstallType::PROTON
                } else {
                    install_type = InstallType::CUSTOM
                }
                Ok(InstalledVersion {
                    name: row.get(0)?,
                    location: row.get(1)?,
                    install_type,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(entries)
    }

    pub fn update_entry(
        &self,
        entry_name: &str,
        new_entry: &InstalledVersion,
    ) -> Result<(), String> {
        Ok(())
    }
}
