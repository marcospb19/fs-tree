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
