use std::path::{Path, PathBuf};

use file_type_enum::FileType;

use crate::{error::*, fs};

/// Follow symlink at `path` in one level, and return the new path.
///
/// # Errors:
/// - If `path` does not exist
/// - If `path` is not a symlink
/// - If `Io::Error` from `fs::read_link(path)`
pub fn symlink_follow<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(Error::NotFoundError(path.to_path_buf()));
        // "while trying to read symlink target path",
    }

    if !FileType::from_path(path)?.is_symlink() {
        return Err(Error::NotASymlinkError(path.to_path_buf()));
        // "while trying to read symlink target path",
    }

    let target = fs::read_link(&path)?;

    Ok(target)
}
