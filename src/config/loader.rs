use crate::{
    config::structs::Config,
    event::{log::Logger, notify::send_notify},
};
use directories::ProjectDirs;
use std::{fs, io, path::PathBuf, sync::Arc};

pub fn try_get_config_path() -> io::Result<PathBuf> {
    let dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
        .ok_or_else(|| io::Error::other("Failed to get config directory"))?;

    let path = dirs.config_dir().join("config.toml");

    if path.is_file() {
        Ok(path)
    } else {
        Err(io::Error::other("Failed to get default config path"))
    }
}

fn try_get_custom_path(custom_path: &Option<PathBuf>) -> io::Result<PathBuf> {
    match custom_path {
        Some(path) => {
            if path.is_file() {
                Ok(path.clone())
            } else {
                Err(io::Error::other("Failed to get custom config path"))
            }
        }
        None => Err(io::Error::other("Failed to get custom config path")),
    }
}

pub fn load_config(custom_path: &Option<PathBuf>, logger: Arc<Logger>) -> Config {
    let path: Option<PathBuf> = match try_get_custom_path(custom_path) {
        Ok(path) => Some(path),
        Err(e) => {
            logger.info(e);
            match try_get_config_path() {
                Ok(path) => {
                    logger.info("Fallback to default config path");
                    send_notify("Fallback to default config path");
                    Some(path)
                }
                Err(e) => {
                    logger.info(e);
                    None
                }
            }
        }
    };

    match path {
        Some(conf) => match fs::read_to_string(&conf) {
            Ok(content) => match toml::from_str(&content) {
                Ok(v) => {
                    logger.trace("Config Load Successfull");
                    send_notify("Config Load Successfull");
                    v
                }
                Err(e) => {
                    logger.warn(format!("Config parse error: {e}"));
                    send_notify(format!("Config parse error\n{e}").as_str());
                    Config::default()
                }
            },
            Err(e) => {
                logger.warn(format!("Config parse error: {e}"));
                send_notify(format!("Config parse error\n{e}").as_str());
                Config::default()
            }
        },
        None => {
            logger.info("Fallback to default config setting");
            send_notify("Fallback to default config setting");
            Config::default()
        }
    }
}
