use std::{error, fmt, io, path::PathBuf, result};

pub type Result<T> = result::Result<T, FSError>;

#[derive(Debug)]
pub enum FSError {
    ReadError {
        path: PathBuf,
        source: io::Error,
        context: &'static str,
    },
    WriteError {
        path: PathBuf,
        source: io::Error,
        context: &'static str,
    },
    NotFoundError {
        path: PathBuf,
    },
    NotADirectoryError {
        path: PathBuf,
    },
    NotASymlinkError {
        path: PathBuf,
    },
    // IoError(io::Error),
}

use FSError::*;

impl error::Error for FSError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ReadError { source, .. } | WriteError { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Ready for displaying errors to end users!
/// Format:
///     "error name: more details: more details"
impl fmt::Display for FSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError { source, .. } => {
                write!(f, "Read error: ")?;
                source.fmt(f)
            },
            WriteError { source, .. } => {
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
        }
    }
}

// impl From<io::Error> for FSError {
//     fn from(err: io::Error) -> Self {
//         FSError::IoError(err)
//     }
// }
