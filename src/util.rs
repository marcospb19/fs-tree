use crate::{error::*, file::File, file_type::FileType};

use std::{
    fs,
    path::{Path, PathBuf},
};

/// Fill a Vec with our own File struct
pub fn collect_directory_children<T>(
    path: impl AsRef<Path>,
    follow_symlinks: bool,
) -> FSResult<Vec<File<T>>> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(FSError::new(
            FSErrorKind::NotFoundError,
            path.into(),
            "while trying to read directory content",
        ));
    }

    if !FileType::<T>::from_path_shallow(&path, follow_symlinks)?.is_dir() {
        return Err(FSError::new(
            FSErrorKind::NotADirectoryError,
            path.into(),
            "while trying to read directory content",
        ));
    }

    let dirs = fs::read_dir(&path);
    let dirs = dirs.map_err(|source| {
        FSError::new(
            FSErrorKind::ReadError(source),
            path.into(),
            "while trying to read directory content",
        )
    })?;

    let mut children = vec![];
    for entry in dirs {
        let entry = entry.map_err(|source| {
            FSError::new(
                FSErrorKind::ReadError(source),
                path.into(),
                "error while reading directory for specific entry",
            )
        })?;

        let file = File::new_from_path(&entry.path(), follow_symlinks)?;
        children.push(file);
    }
    Ok(children)
}

/// Follow symlink only one level
pub fn symlink_target<T, P: AsRef<Path>>(path: P) -> FSResult<PathBuf> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(FSError::new(
            FSErrorKind::NotFoundError,
            path.into(),
            "while trying to read symlink target path",
        ));
    }

    //
    if !FileType::<T>::from_path_shallow(path, false)?.is_symlink() {
        return Err(FSError::new(
            FSErrorKind::NotASymlinkError,
            path.into(),
            "while trying to read symlink target path",
        ));
    }

    let target = fs::read_link(&path);
    let target = target.map_err(|source| {
        FSError::new(
            FSErrorKind::ReadError(source),
            path.into(),
            "while trying to read symlink target path",
        )
    })?;

    Ok(target)
}

/// Used by FileType `from_path*` function
pub fn fs_filetype_from_path(
    path: impl AsRef<Path>,
    follow_symlink: bool,
) -> FSResult<fs::FileType> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(FSError::new(FSErrorKind::NotFoundError, path.into(), ""));
    }

    let metadata_function = if follow_symlink {
        fs::metadata
    } else {
        fs::symlink_metadata
    };

    let metadata = metadata_function(path);
    let metadata = metadata.map_err(|source| {
        FSError::new(
            FSErrorKind::ReadError(source),
            path.to_path_buf(),
            "Unable to gather type information of file at",
        )
    })?;

    Ok(metadata.file_type())
}
