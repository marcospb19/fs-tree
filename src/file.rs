use crate::{error::*, file_type::FileType};

use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct File {
    pub path: PathBuf,
    pub file_type: FileType,
}

impl File {
    pub fn new(path: impl AsRef<Path>, file_type: FileType) -> Self {
        let path = path.as_ref().to_path_buf();
        File { path, file_type }
    }

    pub fn from_path(path: impl AsRef<Path>, follow_symlinks: bool) -> Result<Self> {
        let file_type = FileType::from_path(&path, follow_symlinks)?;
        let result = File::new(path, file_type);

        Ok(result)
    }
}
