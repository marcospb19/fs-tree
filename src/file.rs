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

/// Similar to FileType, but no enum variant inner members.
#[derive(Debug)]
pub enum FlatFileType {
    File,
    Directory,
    SymbolicLink,
}

impl File {
    pub fn new(path: PathBuf, file_type: FileType) -> Self {
        File { path, file_type }
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let kk = File {
            path: path.as_ref().to_path_buf(),
            file_type: FileType::default(),
        };

        Ok(kk)
    }
}

impl FileType {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        // Reuse FlatFileType code, but assing the inner members after
        let file_type = FlatFileType::from_path(&path)?;

        // If File, add nothing,
        // If Directory, add `children`,
        // If SymbolicLink, add `target_path`.
        let result = match file_type {
            FlatFileType::File => FileType::File,
            FlatFileType::Directory => FileType::Directory {
                children: collect_files_from_directory_path(path)?,
            },
            FlatFileType::SymbolicLink => FileType::SymbolicLink {
                target_path: get_symlink_target_from_path(path)?,
            },
        };
        Ok(result)
    }
}

impl FlatFileType {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let metadata_file_type = get_metadata_from_path(&path)?.file_type();

        let result = if metadata_file_type.is_file() {
            FlatFileType::File
        } else if metadata_file_type.is_dir() {
            FlatFileType::Directory
        } else if metadata_file_type.is_symlink() {
            FlatFileType::SymbolicLink
        } else {
            panic!();
        };
        Ok(result)
    }
}

/// Fill a Vec with our own File struct
pub fn collect_files_from_directory_path(path: impl AsRef<Path>) -> Result<Vec<File>> {
    if let FlatFileType::Directory = FlatFileType::from_path(&path)? {
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
    let asd = fs::read_link(&path).map_err(|err| DotaoError::ReadError {
        path: path.as_ref().to_path_buf(),
        source: err,
    })?;

    Ok(asd)
}

/// Used by FileType and FlatFileType `from_path` function.
pub fn get_metadata_from_path(path: impl AsRef<Path>) -> Result<fs::Metadata> {
    let path = path.as_ref();
    let metadata = path.metadata().map_err(|err| DotaoError::ReadError {
        path: path.to_path_buf(),
        source: err,
    })?;

    Ok(metadata)
}

impl Default for FileType {
    fn default() -> Self {
        FileType::File
    }
}

impl Default for FlatFileType {
    fn default() -> Self {
        FlatFileType::File
    }
}
