// Should the functions in here use lazy_static?
use std::{
    env,
    ffi::{CStr, OsStr},
    fs, mem,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    ptr,
};

use libc::{self, c_char};

use crate::error;

pub fn load_groups_from_path(_path: impl AsRef<Path>) -> tsml::Groups {
    let mut groups = tsml::Groups::from_path("dotao.tsml").unwrap();
    // Remove main group please
    if let Some(main_group) = groups.map.remove("main") {
        if !main_group.is_empty() {
            error!("This should be empty");
        }
    }
    groups
}

pub fn current_dir() -> PathBuf {
    env::current_dir()
        .unwrap_or_else(|err| error!("Failed to read curent directory path: '{}'.", err))
}

pub fn home_dir() -> PathBuf {
    unsafe fn char_ptr_to_path_buf(ptr: *mut c_char) -> PathBuf {
        OsStr::from_bytes(CStr::from_ptr(ptr).to_bytes()).into()
    }

    // Check env var, otherwise, call libc::getpwuid_r
    env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| {
            let mut buf = [0; 4096];
            let mut result = ptr::null_mut();
            let mut passwd: libc::passwd = unsafe { mem::zeroed() };

            let getpwuid_r_code = unsafe {
                libc::getpwuid_r(
                    libc::getuid(),
                    &mut passwd,
                    buf.as_mut_ptr(),
                    buf.len(),
                    &mut result,
                )
            };
            // If success
            if getpwuid_r_code == 0 && !result.is_null() {
                let home_dir = unsafe { char_ptr_to_path_buf(passwd.pw_dir) };
                Some(home_dir)
            } else {
                None
            }
        })
        .unwrap_or_else(|| error!("Unable to find HOME dir. Try setting the $HOME env var."))
}

// Opt for ~/.config/dotao/config.toml before ./config.toml
pub fn config_location() -> Option<PathBuf> {
    let local_config = current_dir().join("config.toml");
    let home_config_location = home_dir().join(".config/dotao/config.toml");

    local_config
        .exists()
        .then_some(local_config)
        .or_else(|| home_config_location.exists().then_some(home_config_location))
}

// Always should return Some if config_location does also return Some
pub fn load_config() -> Option<toml::Value> {
    let config_path = config_location()?;
    let text = fs::read_to_string(&config_path).unwrap_or_else(|err| {
        error!("Error while trying to read config file at '{}': {}.", to_utf(config_path), err)
    });

    text.parse::<toml::Value>().unwrap_or_else(|err| {
        error!("Error while trying to parse toml config file at 'config.toml': {}", err)
    });
    todo!();
}

fn toml_type_to_string(value: &toml::Value) -> String {
    match value {
        toml::Value::Array(..) => "Array",
        toml::Value::Boolean(..) => "Boolean",
        toml::Value::Datetime(..) => "Datetime",
        toml::Value::Float(..) => "Float",
        toml::Value::Integer(..) => "Integer",
        toml::Value::String(..) => "String",
        toml::Value::Table(..) => "Table",
    }
    .to_string()
}

pub fn backup_dir() -> PathBuf {
    load_config()
        .and_then(|config| {
            config.get("backup_dir").map(|backup_dir_value| {
                let backup_dir_value = backup_dir_value.as_str().unwrap_or_else(|| {
                    error!(
                        "Error: 'backup_dir' variable at '{}' should be of type String, instead, it's of type {}.",
                        // Safe, cause load_config.is_some(), maybe rework this?
                        to_utf(config_location().unwrap()),
                        toml_type_to_string(backup_dir_value)
                    )
                });
                PathBuf::from(backup_dir_value)

            })
        })
        .unwrap_or_else(|| PathBuf::from(".."))
}

pub fn to_utf(str: impl AsRef<OsStr>) -> String {
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
