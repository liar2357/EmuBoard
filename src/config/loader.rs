use crate::config::structs::Config;
use directories::ProjectDirs;
use std::{fs, io, path::PathBuf};

fn config_path() -> io::Result<PathBuf> {
    let dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
        .ok_or_else(|| io::Error::other("Failed to get config directory"))?;

    Ok(dirs.config_dir().join("config.toml"))
}

pub fn load_config() -> Config {
    let path = match config_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Config path error: {e}");
            return Config::default();
        }
    };

    match fs::read_to_string(&path) {
        Ok(content) => toml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Config parse error: {e}");
            Config::default()
        }),
        Err(_) => Config::default(),
    }
}
