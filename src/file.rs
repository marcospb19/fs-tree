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

    pub fn from_path(path: &impl AsRef<Path>, follow_symlinks: bool) -> Result<Self> {
        let file_type = FileType::from_path(&path, follow_symlinks)?;
        let path = path.as_ref().to_path_buf();
        let result = File::new(path, file_type);

        Ok(result)
    }
}

impl FileType {
    pub fn from_path(path: &impl AsRef<Path>, follow_symlinks: bool) -> Result<Self> {
        let fs_file_type = fs_filetype_from_path(&path, follow_symlinks)?;

        // Is file, directory, or symlink
        let result = if fs_file_type.is_file() {
            FileType::File
        } else if fs_file_type.is_dir() {
            let children = collect_files_from_directory(&path, follow_symlinks)?;
            FileType::Directory { children }
        } else if fs_file_type.is_symlink() {
            let target_path = get_symlink_target_from_path(path)?;
            FileType::SymbolicLink { target_path }
        } else {
            todo!("Other file types.")
        };

        Ok(result)
    }

    pub fn from_path_shallow(path: &impl AsRef<Path>, follow_symlink: bool) -> Result<Self> {
        let fs_file_type = fs_filetype_from_path(&path, follow_symlink)?;

        // Is file, directory, or symlink
        let result = {
            if fs_file_type.is_file() {
                FileType::File
            } else if fs_file_type.is_dir() {
                FileType::Directory { children: vec![] }
            } else if fs_file_type.is_symlink() {
                FileType::SymbolicLink {
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

    pub fn is_directory(&self) -> bool {
        matches!(self, FileType::Directory { .. })
    }

    pub fn is_symbolic_link(&self) -> bool {
        matches!(self, FileType::SymbolicLink { .. })
    }
}

/// Fill a Vec with our own File struct
pub fn collect_files_from_directory(
    path: &impl AsRef<Path>,
    follow_symlinks: bool,
) -> Result<Vec<File>> {
    let path = path.as_ref().to_path_buf();
    if !FileType::from_path_shallow(&path, follow_symlinks)?.is_directory() {
        return Err(DotaoError::NotADirectory);
    }

    let dirs = fs::read_dir(&path).map_err(|source| DotaoError::ReadError {
        path: path.clone(),
        source,
    })?;

    let mut children = vec![];
    for entry in dirs {
        let entry = entry.map_err(|source| DotaoError::ReadError {
            path: path.clone(),
            source,
        })?;

        let file = File::from_path(&entry.path(), follow_symlinks)?;
        children.push(file);
    }
    Ok(children)
}

/// Follow symlink one level
pub fn get_symlink_target_from_path(path: &impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(DotaoError::NotFoundInFilesystem);
    }

    let target = fs::read_link(&path).map_err(|source| DotaoError::ReadError {
        path: path.to_path_buf(),
        source,
    })?;

    Ok(target)
}

/// Used by FileType `from_path*` function.
pub fn fs_filetype_from_path(
    path: &impl AsRef<Path>,
    follow_symlink: bool,
) -> Result<fs::FileType> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(DotaoError::NotFoundInFilesystem);
    }

    let metadata_function = if follow_symlink {
        fs::metadata
    } else {
        fs::symlink_metadata
    };

    let metadata = metadata_function(path).map_err(|source| DotaoError::ReadError {
        path: path.to_path_buf(),
        source,
    })?;

    Ok(metadata.file_type())
}

impl Default for FileType {
    fn default() -> Self {
        FileType::File
    }
}
