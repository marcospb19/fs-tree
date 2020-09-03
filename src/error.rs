use std::{error, fmt, io, path::PathBuf, result};

pub type Result<T> = result::Result<T, DotaoError>;

/// DotaoError covers all possible errors from this library
#[derive(Debug)]
pub enum DotaoError {
    ReadError { path: PathBuf, source: io::Error },
    NotFoundInFilesystem,
    NotADirectory,
}

impl error::Error for DotaoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            DotaoError::ReadError { source, .. } => Some(source),
            DotaoError::NotFoundInFilesystem => None,
            DotaoError::NotADirectory => None,
        }
    }
}

impl fmt::Display for DotaoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DotaoError::ReadError { source, .. } => source.fmt(f),
            DotaoError::NotFoundInFilesystem => write!(f, "File not found"),
            DotaoError::NotADirectory => write!(f, "File is not a directory"),
        }
    }
}
