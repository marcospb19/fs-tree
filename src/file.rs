use crate::{error::*, file_type::FileType};

use std::{
    fmt,
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Clone)]
pub struct File {
    pub path: PathBuf,
    pub file_type: FileType,
}

impl File {
    pub fn new(path: PathBuf, file_type: FileType) -> Self {
        File { path, file_type }
    }

    pub fn from_path(path: impl AsRef<Path>, follow_symlinks: bool) -> Result<Self> {
        let file_type = FileType::from_path(&path, follow_symlinks)?;
        let path = path.as_ref().to_path_buf();
        let result = File::new(path, file_type);

        Ok(result)
    }
}

impl Default for FileType {
    fn default() -> Self {
        FileType::File
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileType::File => write!(f, "file"),
            FileType::Directory { .. } => write!(f, "directory"),
            FileType::Symlink { .. } => write!(f, "symbolic link"),
        }
    }
}
