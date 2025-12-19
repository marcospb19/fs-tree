use std::{error, fmt, io, path::PathBuf};

use file_type_enum::FileType;

/// An alias for `Result<T, fs_tree::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// An enum for all errors generated in the `fs-tree` crate.
#[derive(Debug)]
pub enum Error {
    /// Expected directory, but file type differs.
    NotADirectory(PathBuf),
    /// Expected symlink, but file type differs.
    NotASymlink(PathBuf),
    /// Unsupported file type found.
    UnexpectedFileType(FileType, PathBuf),
    /// An error with reading or writing.
    Io(io::Error),
}

use Error::*;

impl Error {
    /// The path related to this error, if any.
    pub fn path(&self) -> Option<&PathBuf> {
        match self {
            NotADirectory(path) | NotASymlink(path) | UnexpectedFileType(_, path) => Some(path),
            Io(..) => None,
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Io(source) => Some(source),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FsError: ")?;

        match self {
            NotADirectory(..) => write!(f, "not a directory"),
            NotASymlink(..) => write!(f, "not a symlink"),
            UnexpectedFileType(..) => write!(f, "unexpected file type"),
            Io(inner) => inner.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
