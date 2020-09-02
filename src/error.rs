use std::{error, fmt, io, result};

pub type Result<T> = result::Result<T, DotaoError>;

/// DotaoError covers all possible errors from this library
#[derive(Debug)]
pub enum DotaoError {
    ReadError { source: io::Error },
    IoError(io::Error),
    Other,
}

impl error::Error for DotaoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            DotaoError::ReadError { source } => Some(source),
            DotaoError::IoError(_) => None,
            DotaoError::Other => None,
        }
    }
}

impl fmt::Display for DotaoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DotaoError::ReadError { .. } => write!(f, "temporary error_message"),
            DotaoError::IoError(_) => write!(f, "temporary error_message"),
            DotaoError::Other => write!(f, "temporary error_message"),
        }
    }
}

impl From<io::Error> for DotaoError {
    fn from(err: io::Error) -> Self {
        DotaoError::IoError(err)
    }
}
