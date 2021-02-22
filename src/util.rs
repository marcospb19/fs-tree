// Should the functions in here use lazy_static?
use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use crate::error;

pub fn current_dir() -> PathBuf {
    env::current_dir()
        .unwrap_or_else(|err| error!("Failed to read curent directory path: '{}'.", err))
}

pub fn home_dir() -> PathBuf {
    env::var("HOME").expect("$HOME not found").into()
}

// Opt for ~/.config/dotao/config.toml before ./config.toml
pub fn config_location() -> Option<PathBuf> {
    let local_config = current_dir().join("config.toml");
    let home_config_location = home_dir().join(".config/dotao/config.toml");

    local_config
        .exists()
        .then_some(local_config)
        .or(home_config_location.exists().then_some(home_config_location))
}

pub fn load_config() -> Option<toml::Value> {
    let config_path: PathBuf = config_location()?;
    let text = fs::read_to_string(&config_path).unwrap_or_else(|err| {
        error!("Error while trying to read toml config file at '{}': {}.", to_uft(config_path), err)
    });
    todo!()

    // toml::from_str(&text).ok()
}

pub fn backup_dir() -> PathBuf {
    use toml::Value;

    let value = "foo = 'bar'".parse::<Value>().unwrap();

    assert_eq!(value["foo"].as_str(), Some("bar"));
    todo!()
}

pub fn to_uft(str: impl AsRef<OsStr>) -> String {
    let text = format!("{:?}", str.as_ref());
    text.trim_matches('"').to_string()
}

fn _is_currently_in_git_repository() -> bool {
    let current_dir = current_dir();
    // Used to traverse
    let mut path: &Path = &current_dir;
    loop {
        if path.join(".git").exists() {
            return true;
        } else if let Some(parent) = path.parent() {
            path = parent;
        } else {
            return false;
        }
    }
}

pub fn is_in_dotfiles_folder() -> bool {
    let current_dir = current_dir();
    match current_dir.file_name() {
        None => false,
        Some(file_name) => file_name == Path::new("dotfiles"),
    }
}
