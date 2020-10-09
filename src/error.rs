use std::{error, fmt, io, path::PathBuf, result};

pub type Result<T> = result::Result<T, FSError>;

#[derive(Debug)]
pub struct FSError {
    context: &'static str, // Optional aditional context for debugging purposes, can be empty
    kind: FSErrorKind,
    path: PathBuf,
}

/// Ready for displaying errors to end users!
///
/// ### Format:
/// ```txt
/// Read error: Unable to read directory content: 'path/to/file'
/// ```
///
/// Note that all our functions execute excessive checks before
impl FSError {
    pub(crate) fn new(kind: FSErrorKind, path: PathBuf, context: &'static str) -> Self {
        Self {
            context,
            kind,
            path,
        }
    }

    pub fn context(&self) -> &'static str {
        self.context
    }

    pub fn kind(&self) -> &FSErrorKind {
        &self.kind
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[derive(Debug)]
pub enum FSErrorKind {
    ReadError(io::Error),
    WriteError(io::Error),
    NotFoundError,
    NotADirectoryError,
    NotASymlinkError,
}

use FSErrorKind::*;

impl error::Error for FSError {
    // Should this return Option<&io::Error> instead? nope, let's keep it like this
    // while there's no logistic link error...
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ReadError(io_source) | WriteError(io_source) => Some(io_source),
            _ => None,
        }
    }
}

impl fmt::Display for FSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ReadError(ref io_err) => {
                write!(f, "Read error: ")?;
                io_err.fmt(f)
            },
            WriteError(ref io_err) => {
                write!(f, "Write error: ")?;
                io_err.fmt(f)
            },
            NotFoundError => write!(f, "Error: file not found: "),
            NotADirectoryError => write!(f, "Error: not a directory: "),
            NotASymlinkError => write!(f, "Error: not a symlink: "),
        }?;

        if !self.context.is_empty() {
            write!(f, "{}: ", self.context)?;
        }

        write!(f, "from '{}'.", self.path.display())
    }
}

// impl From<io::Error> for FSError {
//     fn from(err: io::Error) -> Self {
//         FSError::IoError(err)
//     }
// }
