use std::path::PathBuf;

pub fn get_cache() -> PathBuf {
    let home = home::home_dir().expect("No home directory.");
    home.join(".cache/license")
}
