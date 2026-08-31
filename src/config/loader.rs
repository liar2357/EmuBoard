use crate::{config::structs::Config, event::notify::send_notify};
use directories::ProjectDirs;
use std::{fs, io, path::PathBuf};

pub fn try_get_config_path() -> io::Result<PathBuf> {
    let dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
        .ok_or_else(|| io::Error::other("Failed to get config directory"))?;

    Ok(dirs.config_dir().join("config.toml"))
}

pub fn load_config() -> Config {
    let path = match try_get_config_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Config path error: {e}");
            return Config::default();
        }
    };

    match fs::read_to_string(&path) {
        Ok(content) => match toml::from_str(&content) {
            Ok(v) => {
                send_notify("Config Load Successfull");
                v
            }
            Err(e) => {
                send_notify(format!("Config parse error\n{e}").as_str());
                Config::default()
            }
        },
        Err(e) => {
            send_notify(format!("Config parse error\n{e}").as_str());
            Config::default()
        }
    }
}
