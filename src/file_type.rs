use crate::{collect_directory_chidren, error::*, fs_filetype_from_path, symlink_target, File};

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum FileType {
    File,
    Directory { children: Vec<File> },
    Symlink { target_path: PathBuf },
}

impl FileType {
    pub fn from_path(path: impl AsRef<Path>, follow_symlinks: bool) -> Result<Self> {
        let fs_file_type = fs_filetype_from_path(&path, follow_symlinks)?;

        // Is file, directory, or symlink
        let result = if fs_file_type.is_file() {
            FileType::File
        } else if fs_file_type.is_dir() {
            let children = collect_directory_chidren(&path, follow_symlinks)?;
            FileType::Directory { children }
        } else if fs_file_type.is_symlink() {
            let target_path = symlink_target(path)?;
            FileType::Symlink { target_path }
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
                FileType::File
            } else if fs_file_type.is_dir() {
                FileType::Directory { children: vec![] }
            } else if fs_file_type.is_symlink() {
                FileType::Symlink {
                    target_path: PathBuf::new(),
                }
            } else {
                todo!("Other file types.")
            }
        };
        Ok(result)
    }

    pub fn is_file(&self) -> bool {
        matches!(self, FileType::File)
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, FileType::Directory { .. })
    }

    pub fn is_symlink(&self) -> bool {
        matches!(self, FileType::Symlink { .. })
    }
}
