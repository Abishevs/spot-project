use std::fs::create_dir_all;
use dirs::home_dir;
use std::path::PathBuf;
use std::io;

pub fn get_db_path() -> PathBuf {
    let mut path = home_dir().expect("Home directory not found");
    path.push(".config");
    path.push("spot");
    path.push("cache.db");
    path
}

pub fn create_config_dir() -> io::Result<PathBuf> {
    if let Some(mut config_path) = home_dir() {
        config_path.push(".config");
        config_path.push("spot");

        create_dir_all(&config_path)?;
        Ok(config_path)

    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))
    }
}
