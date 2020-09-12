use std::{error, fmt, io, path::PathBuf, result};

pub type Result<T> = result::Result<T, DotaoError>;

/// DotaoError covers all possible errors from this library
#[derive(Debug)]
pub enum DotaoError {
    ReadError { path: PathBuf, source: io::Error },
    NotFoundInFilesystem,
    NotADirectory,
}

use DotaoError::*;

impl error::Error for DotaoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ReadError { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl fmt::Display for DotaoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError { source, .. } => source.fmt(f),
            NotFoundInFilesystem => write!(f, "File not found"),
            NotADirectory => write!(f, "File is not a directory"),
        }
    }
}
