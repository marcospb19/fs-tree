use crate::{
    error::*,
    util::{collect_directory_children, fs_filetype_from_path, symlink_target},
    File,
};

use std::{
    fmt,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FileType {
    Regular,
    Directory(Vec<File>),
    Symlink(PathBuf),
}

impl FileType {
    pub fn from_path(path: impl AsRef<Path>, follow_symlinks: bool) -> Result<Self> {
        let fs_file_type = fs_filetype_from_path(&path, follow_symlinks)?;

        // Is file, directory, or symlink
        let result = if fs_file_type.is_file() {
            FileType::Regular
        } else if fs_file_type.is_dir() {
            let children = collect_directory_children(&path, follow_symlinks)?;
            FileType::Directory(children)
        } else if fs_file_type.is_symlink() {
            let target_path = symlink_target(path)?;
            FileType::Symlink(target_path)
        } else {
            todo!("Other file types.")
        };

        Ok(result)
    }

    pub fn from_path_shallow(path: impl AsRef<Path>, follow_symlink: bool) -> Result<Self> {
        let fs_file_type = fs_filetype_from_path(&path, follow_symlink)?;

        // Is file, directory, or symlink
        let result = {
            if fs_file_type.is_file() {
                FileType::Regular
            } else if fs_file_type.is_dir() {
                FileType::Directory(vec![])
            } else if fs_file_type.is_symlink() {
                FileType::Symlink(PathBuf::new())
            } else {
                todo!("Other file types.")
            }
        };
        Ok(result)
    }

    pub fn is_file(&self) -> bool {
        matches!(self, FileType::Regular)
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, FileType::Directory { .. })
    }

    pub fn is_symlink(&self) -> bool {
        matches!(self, FileType::Symlink { .. })
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::Regular
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileType::Regular => write!(f, "file"),
            FileType::Directory { .. } => write!(f, "directory"),
            FileType::Symlink { .. } => write!(f, "symbolic link"),
        }
    }
}
