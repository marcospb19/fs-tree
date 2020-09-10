use std::{error, fmt, io, path::PathBuf, result};

pub type Result<T> = result::Result<T, DotaoError>;

/// DotaoError covers all possible errors from this library
#[derive(Debug)]
pub enum DotaoError {
    ReadError { path: PathBuf, source: io::Error },
    UnableToEnterDirectory { path: PathBuf, source: io::Error },
    NotFoundInFilesystem,
    NotADirectory,
    LinkError(DotaoLinkError),
}

use DotaoError::*;

impl error::Error for DotaoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ReadError { source, .. } | UnableToEnterDirectory { source, .. } => Some(source),
            LinkError(source) => Some(source),
            _ => None,
        }
    }
}

impl fmt::Display for DotaoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError { source, .. } => source.fmt(f),
            UnableToEnterDirectory { .. } => write!(f, "Unable to enter directory"),
            NotFoundInFilesystem => write!(f, "File not found"),
            NotADirectory => write!(f, "File is not a directory"),
            LinkError(source) => {
                write!(f, "Link error: ")?;
                source.fmt(f)
            },
        }
    }
}

#[derive(Debug)]
pub enum DotaoLinkError {
    A,
}

use DotaoLinkError::*;

impl error::Error for DotaoLinkError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            A => None,
        }
    }
}

impl fmt::Display for DotaoLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            A => write!(f, ""),
        }
    }
}
