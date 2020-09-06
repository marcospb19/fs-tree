use crate::{
    error::*,
    file::{File, FlatFileType},
};

use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct DotfileGroup {
    pub starting_path: PathBuf,
    pub files: Vec<File>,
}

impl DotfileGroup {
    pub fn new(path: PathBuf, files: Vec<File>) -> Self {
        DotfileGroup {
            starting_path: path,
            files,
        }
    }

    pub fn from_directory_path(path: impl AsRef<Path>) -> Result<Self> {
        if !path.as_ref().exists() {
            return Err(DotaoError::NotFoundInFilesystem);
        }

        match FlatFileType::from_path(path)? {
            FlatFileType::Directory => {},
            _ => return Err(DotaoError::NotADirectory),
        }

        unimplemented!();

        Ok(DotfileGroup::default())
    }
}
