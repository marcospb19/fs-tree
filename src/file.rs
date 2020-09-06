use crate::error::*;

use std::{
    fs,
    path::{Path, PathBuf},
};

/// Dotao file representation, if it's a directory, then structures a tree.
#[derive(Debug, Default)]
pub struct File {
    pub path: PathBuf,
    pub file_type: FileType,
}

/// Internal representation of a file type, simplified, only 3 variants.
#[derive(Debug)]
pub enum FileType {
    File,
    Directory { children: Vec<File> },
    SymbolicLink { target_path: PathBuf },
}

impl File {
    pub fn new(path: PathBuf, file_type: FileType) -> Self {
        File { path, file_type }
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let file_type = FileType::from_path(&path)?;

        let result = File::new(path.as_ref().to_path_buf(), file_type);

        Ok(result)
    }
}

impl FileType {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let metadata = get_symlink_metadata_from_path(&path)?;

        // Is file, directory, or symlink
        let result = if metadata.is_file() {
            FileType::File
        } else if metadata.is_dir() {
            let children = collect_files_from_directory_path(path)?;
            FileType::Directory { children }
        } else {
            let target_path = get_symlink_target_from_path(path)?;
            FileType::SymbolicLink { target_path }
        };

        Ok(result)
    }

    pub fn is_file(&self) -> bool {
        match self {
            FileType::File => true,
            _ => false,
        }
    }

    pub fn is_directory(&self) -> bool {
        match self {
            FileType::Directory { .. } => true,
            _ => false,
        }
    }

    pub fn is_symbolic_link(&self) -> bool {
        match self {
            FileType::SymbolicLink { .. } => true,
            _ => false,
        }
    }
}

/// Fill a Vec with our own File struct
pub fn collect_files_from_directory_path(path: impl AsRef<Path>) -> Result<Vec<File>> {
    if !get_symlink_metadata_from_path(&path)?.is_dir() {
        return Err(DotaoError::NotADirectory);
    }

    let dirs = fs::read_dir(&path).map_err(|err| DotaoError::ReadError {
        path: path.as_ref().to_path_buf(),
        source: err,
    })?;

    let mut children = vec![];
    for entry in dirs {
        let entry = entry.map_err(|err| DotaoError::ReadError {
            path: path.as_ref().to_path_buf(),
            source: err,
        })?;

        let file = File::from_path(entry.path())?;
        children.push(file);
    }
    Ok(children)
}

/// Follow symlink one level
pub fn get_symlink_target_from_path(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(DotaoError::NotFoundInFilesystem);
    }

    let result = fs::read_link(&path).map_err(|err| DotaoError::ReadError {
        path: path.to_path_buf(),
        source: err,
    })?;

    Ok(result)
}

/// Used by FileType and FlatFileType `from_path` function.
pub fn get_symlink_metadata_from_path(path: impl AsRef<Path>) -> Result<fs::Metadata> {
    let path = path.as_ref().to_path_buf();

    if !path.exists() {
        return Err(DotaoError::NotFoundInFilesystem);
    }

    let metadata = path
        .metadata()
        .map_err(|err| DotaoError::ReadError { path, source: err })?;

    Ok(metadata)
}

impl Default for FileType {
    fn default() -> Self {
        FileType::File
    }
}
