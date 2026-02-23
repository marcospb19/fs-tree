use std::{error, fmt, io, path::PathBuf};

use file_type_enum::FileType;

/// An alias for `Result<T, fs_tree::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// An enum for all errors generated in the `fs-tree` crate.
#[derive(Debug)]
pub enum Error {
    /// Expected directory, but file type differs.
    NotADirectory(PathBuf),
    /// Expected regular file, but file type differs.
    NotARegularFile(PathBuf),
    /// Expected symlink, but file type differs.
    NotASymlink(PathBuf),
    /// Symlink exists but points to a different target than expected.
    SymlinkTargetMismatch {
        /// The path to the symlink.
        path: PathBuf,
        /// The expected target.
        expected: PathBuf,
        /// The actual target found.
        found: PathBuf,
    },
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
            NotADirectory(path)
            | NotARegularFile(path)
            | NotASymlink(path)
            | SymlinkTargetMismatch { path, .. }
            | UnexpectedFileType(_, path) => Some(path),
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
        match self {
            NotADirectory(path) => write!(f, "not a directory: {}", path.display()),
            NotARegularFile(path) => write!(f, "not a regular file: {}", path.display()),
            NotASymlink(path) => write!(f, "not a symlink: {}", path.display()),
            SymlinkTargetMismatch {
                path,
                expected,
                found,
            } => {
                write!(
                    f,
                    "symlink target mismatch at {}: expected {}, found {}",
                    path.display(),
                    expected.display(),
                    found.display(),
                )
            },
            UnexpectedFileType(file_type, path) => {
                write!(
                    f,
                    "unexpected file type {:?}: {}",
                    file_type,
                    path.display(),
                )
            },
            Io(inner) => inner.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
