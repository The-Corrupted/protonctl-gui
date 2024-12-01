use crate::plib::db::{InstallType, InstalledVersion};

pub fn get_mocked_install_versions(number: u64) -> Vec<InstalledVersion> {
    let b_string = "ProtonGE-".to_owned();
    let mut vec: Vec<InstalledVersion> = Vec::new();
    for x in 0..number {
        let mut install_type = InstallType::WINE;
        if x % 4 == 1 {
            install_type = InstallType::WINE
        } else if x % 4 == 2 {
            install_type = InstallType::PROTON
        } else if x % 4 == 3 {
            install_type = InstallType::UMU
        } else {
            install_type = InstallType::CUSTOM
        }
        let v = InstalledVersion {
            name: format!("{}{}", b_string, x),
            location: "/path/to/the/file".to_owned(),
            install_type,
        };
        vec.push(v);
    }
    vec
}
