use crate::{
    error::*,
    file::{collect_files_from_directory_path, File, FlatFileType},
};

use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct DotfileGroup {
    pub starting_path: PathBuf,
    pub files: Vec<File>,
}

impl DotfileGroup {
    pub fn new(starting_path: PathBuf, files: Vec<File>) -> Self {
        DotfileGroup {
            starting_path,
            files,
        }
    }

    pub fn from_directory_path(path: impl AsRef<Path>) -> Result<Self> {
        if !path.as_ref().exists() {
            return Err(DotaoError::NotFoundInFilesystem);
        } else if !FlatFileType::from_path(&path)?.is_directory() {
            return Err(DotaoError::NotADirectory);
        }

        // Get all chidren from the directory path
        let files = collect_files_from_directory_path(&path)?;

        Ok(DotfileGroup::new(path.as_ref().to_path_buf(), files))
    }
}
