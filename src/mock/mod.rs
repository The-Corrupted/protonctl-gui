use crate::plib::db::InstalledVersion;

pub fn get_mocked_install_versions(number: u64) -> Vec<InstalledVersion> {
    let b_string = "ProtonGE-".to_owned();
    let mut vec: Vec<InstalledVersion> = Vec::new();
    for x in 0..number {
        let v = InstalledVersion {
            name: format!("{}{}", b_string, x),
            location: "/path/to/the/file".to_owned(),
        };
        vec.push(v);
    }
    vec
}
