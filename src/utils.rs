use std::path::{Path, PathBuf};

use file_type_enum::FileType;

use crate::{fs, Error, Result};

/// Follow symlink at `path` just one level, and return the new path.
///
/// # Errors:
/// - If `path` does not exist
/// - If `path` is not a symlink
/// - If `Io::Error` from `fs::read_link(path)`
pub(crate) fn follow_symlink<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();

    if FileType::symlink_read_at(path).is_ok_and(|file| !file.is_symlink()) {
        return Err(Error::NotASymlinkError(path.to_path_buf()));
    }

    let target = fs::read_link(path)?;

    Ok(target)
}
