use crate::FileTypeEnum;
use std::{error, fmt, io, path::PathBuf};

/// Our `Result` type.
pub type FtResult<T> = Result<T, FtError>;

/// Errors generated inside of the `fs-tree` crate.
#[derive(Debug)]
pub enum FtError {
    /// File not found.
    NotFoundError(PathBuf),
    /// Expected directory, but file type differs.
    NotADirectoryError(PathBuf),
    /// Expected symlink, but file type differs.
    NotASymlinkError(PathBuf),
    /// Unsupported file type found.
    UnexpectedFileTypeError(FileTypeEnum, PathBuf),
    /// An error with reading or writing.
    IoError(io::Error),
}

use FtError::*;

impl FtError {
    /// Path to where the error occurred
    pub fn path(&self) -> Option<&PathBuf> {
        match self {
            NotFoundError(path)
            | NotADirectoryError(path)
            | NotASymlinkError(path)
            | UnexpectedFileTypeError(_, path) => Some(path),
            IoError(..) => None,
        }
    }
}

impl error::Error for FtError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            IoError(source) => Some(source),
            _ => None,
        }
    }
}

impl fmt::Display for FtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotFoundError(..) => write!(f, "file not found"),
            NotADirectoryError(..) => write!(f, "not a directory"),
            NotASymlinkError(..) => write!(f, "not a symlink"),
            UnexpectedFileTypeError(..) => write!(f, "unexpected file type"),
            IoError(inner) => inner.fmt(f),
        }
    }
}

impl From<io::Error> for FtError {
    fn from(err: io::Error) -> Self {
        FtError::IoError(err)
    }
}
