use crate::{error::*, File, FileType};

use std::{
    fs,
    path::{Path, PathBuf},
};

/// Fill a Vec with our own File struct
pub fn collect_directory_children(
    path: impl AsRef<Path>,
    follow_symlinks: bool,
) -> Result<Vec<File>> {
    let path = path.as_ref();
    if !FileType::from_path_shallow(&path, follow_symlinks)?.is_dir() {
        return Err(FSError::NotADirectoryError { path: path.into() });
    }

    let dirs = fs::read_dir(&path).map_err(|source| FSError::ReadError {
        path: path.into(),
        source,
        context: "Unable to read directory content",
    })?;

    let mut children = vec![];
    for entry in dirs {
        let entry = entry.map_err(|source| FSError::ReadError {
            path: path.into(),
            source,
            context: "error while reading directory content at entry",
        })?;

        let file = File::from_path(&entry.path(), follow_symlinks)?;
        children.push(file);
    }
    Ok(children)
}

/// Follow symlink one level
pub fn symlink_target(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(FSError::NotFoundError { path: path.into() });
    }

    let target = fs::read_link(&path).map_err(|source| FSError::ReadError {
        path: path.into(),
        source,
        context: "Unable to read link target path",
    })?;

    Ok(target)
}

/// Used by FileType `from_path*` function.
pub fn fs_filetype_from_path(path: impl AsRef<Path>, follow_symlink: bool) -> Result<fs::FileType> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(FSError::NotFoundError { path: path.into() });
    }

    let metadata_function = if follow_symlink {
        fs::metadata
    } else {
        fs::symlink_metadata
    };

    let metadata = metadata_function(path).map_err(|source| FSError::ReadError {
        path: path.to_path_buf(),
        source,
        context: "Unable to gather type information of file at",
    })?;

    Ok(metadata.file_type())
}
