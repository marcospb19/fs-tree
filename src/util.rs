use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use lazy_static::lazy_static;

use crate::error;

lazy_static! {
    pub static ref CURRENT_DIR: PathBuf = env::current_dir()
        .unwrap_or_else(|err| error!("Failed to read curent directory path: '{}'.", err));
}

pub fn bytes_to_uft(asd: impl AsRef<OsStr>) -> String {
    let text = format!("{:?}", asd.as_ref());
    text.trim_matches('"').to_string()
}

pub fn is_currently_in_git_repository() -> bool {
    let current_dir = &CURRENT_DIR;
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
    let current_dir = &CURRENT_DIR;
    match current_dir.file_name() {
        None => false,
        Some(file_name) => file_name == Path::new("dotfiles"),
    }
}
