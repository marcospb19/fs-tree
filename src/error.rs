use std::{error, fmt, io, path::PathBuf};

/// Our `Result` type.
pub type FtResult<T> = Result<T, FtError>;

#[derive(Debug)]
pub enum FtError {
    NotFoundError(PathBuf),
    NotADirectoryError(PathBuf),
    NotASymlinkError(PathBuf),
    UnexpectedFileTypeError(PathBuf),
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
            | UnexpectedFileTypeError(path) => Some(path),
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
