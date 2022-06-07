use std::{
    fs,
    path::{Path, PathBuf},
};

use file_type_enum::FileType as FileTypeEnum;

use crate::error::*;

/// Follow symlink at `path` in one level, and return the new path.
///
/// # Errors:
/// - If `path` does not exist
/// - If `path` is not a symlink
/// - If `Io::Error` from `fs::read_link(path)`
pub fn symlink_target<P: AsRef<Path>>(path: P) -> FtResult<PathBuf> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(FtError::NotFoundError(path.to_path_buf()));
        // "while trying to read symlink target path",
    }

    if !FileTypeEnum::from_path(path)?.is_symlink() {
        return Err(FtError::NotASymlinkError(path.to_path_buf()));
        // "while trying to read symlink target path",
    }

    let target = fs::read_link(&path)?;

    // .map_err(|source| { FtError::new(
    //         FtErrorKind::ReadError(source),
    //         path.into(),
    //         "while trying to read symlink target path",
    //     )
    // })?;

    Ok(target)
}

// /// Used by FileType `from_path*` function
// pub fn fs_filetype_from_path(path: impl AsRef<Path>) -> FtResult<fs::FileType> {
//     let path = path.as_ref();
//     if !path.exists() {
//         return Err(FtError::new(FtErrorKind::NotFoundError, path.into(), ""));
//     }
//     let metadata_function = if follow_symlink { fs::metadata } else { fs::symlink_metadata };
//     let metadata = metadata_function(path);
//     let metadata = metadata.map_err(|source| {
//         FtError::new(
//             FtErrorKind::ReadError(source),
//             path.to_path_buf(),
//             "Unable to gather type information of file at",
//         )
//     })?;
//     Ok(metadata.file_type())
// }
