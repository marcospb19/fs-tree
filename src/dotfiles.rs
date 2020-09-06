use crate::{
    error::*,
    file::{collect_files_from_directory_path, get_symlink_metadata_from_path, File},
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
        let path = path.as_ref();
        if !path.exists() {
            return Err(DotaoError::NotFoundInFilesystem);
        } else if !get_symlink_metadata_from_path(&path)?.is_dir() {
            return Err(DotaoError::NotADirectory);
        }

        // Recursively get all chidren from the directory path
        let files = collect_files_from_directory_path(&path)?;

        let group = DotfileGroup::new(path.to_path_buf(), files);
        Ok(group)
    }
}
