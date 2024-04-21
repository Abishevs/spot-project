use std::fs::create_dir_all;
use dirs::home_dir;
use std::path::PathBuf;
use std::io;

pub fn get_db_path(config_path: &PathBuf) -> PathBuf {
    let db_path = config_path.join("cache.db");
    db_path
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
