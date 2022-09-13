use std::{error, fmt, io, path::PathBuf};

use file_type_enum::FileType;

/// Result for all `fs-tree` crate errors.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors generated inside of the `fs-tree` crate.
#[derive(Debug)]
pub enum Error {
    /// File not found.
    NotFoundError(PathBuf),
    /// Expected directory, but file type differs.
    NotADirectoryError(PathBuf),
    /// Expected symlink, but file type differs.
    NotASymlinkError(PathBuf),
    /// Unsupported file type found.
    UnexpectedFileTypeError(FileType, PathBuf),
    /// An error with reading or writing.
    IoError(io::Error),
}

use Error::*;

impl Error {
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

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            IoError(source) => Some(source),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
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

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}
