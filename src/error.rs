use std::{error, fmt, io, path::PathBuf, result};

pub type Result<T> = result::Result<T, FileStructureError>;

#[derive(Debug)]
pub enum FileStructureError {
    ReadError { source: io::Error },
    WriteError { source: io::Error },
    NotFoundError { path: PathBuf },
    NotADirectoryError { path: PathBuf },
    NotASymlinkError { path: PathBuf },
    IoError(io::Error),
}

use FileStructureError::*;

impl error::Error for FileStructureError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ReadError { source } | WriteError { source } => Some(source),
            _ => None,
        }
    }
}

/// Ready for displaying errors to end users!
/// Format:
///     "error name: more details: more details"
impl fmt::Display for FileStructureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError { source } => {
                write!(f, "Read error: ")?;
                source.fmt(f)
            },
            WriteError { source } => {
                write!(f, "Write error: ")?;
                source.fmt(f)
            },
            NotFoundError { path } => {
                write!(f, "error: ")?;
                path.display().fmt(f)
            },
            NotADirectoryError { path } => {
                write!(f, "error: ")?;
                path.display().fmt(f)
            },
            NotASymlinkError { path } => {
                write!(f, "error: ")?;
                path.display().fmt(f)
            },
            IoError(err) => err.fmt(f), // NotFoundInFilesystem => write!(f, "File not found"),
        }
    }
}

impl From<io::Error> for FileStructureError {
    fn from(err: io::Error) -> Self {
        FileStructureError::IoError(err)
    }
}
