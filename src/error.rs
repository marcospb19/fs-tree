use std::{error, fmt, io, path::PathBuf, result};

/// Dotao's dotao::error::Result<T> = Result<T, DotaoError>
pub type Result<T> = result::Result<T, DotaoError>;

/// DotaoError covers all possible errors from this library
#[derive(Debug)]
pub enum DotaoError {
    LinkError {
        from: PathBuf,
        to: PathBuf,
        source: io::Error,
    },
    ReadError {
        path: PathBuf,
        source: io::Error,
    },
    NotFoundInFilesystem,
    NotADirectory,
}

use DotaoError::*;

impl error::Error for DotaoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ReadError { source, .. } | LinkError { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Display ready for showing errors to users!
/// Error format: "err: more details: more details"
impl fmt::Display for DotaoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError { source, .. } => {
                write!(f, "Read error: ")?;
                source.fmt(f)
            },
            LinkError { source, from, to } => {
                write!(
                    f,
                    "Link error: from '{}' to '{}': ",
                    from.display(),
                    to.display()
                )?;
                source.fmt(f)
            },
            NotFoundInFilesystem => write!(f, "File not found"),
            NotADirectory => write!(f, "File is not a directory"),
        }
    }
}
