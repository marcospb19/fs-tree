use std::{error, fmt, io, path::PathBuf};

/// The only `Result` used in the public API of this crate
pub type FtResult<T> = Result<T, FtError>;

/// The error type for this crate
#[derive(Debug)]
pub struct FtError {
    context: &'static str, // Optional aditional context for debugging purposes, can be empty
    kind: FtErrorKind,
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
impl FtError {
    pub(crate) fn new(kind: FtErrorKind, path: PathBuf, context: &'static str) -> Self {
        Self { context, kind, path }
    }

    /// Description of the error
    pub fn context(&self) -> &'static str {
        self.context
    }

    /// Enum with all possible error variants
    pub fn kind(&self) -> &FtErrorKind {
        &self.kind
    }

    /// Path to where the error occurred
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

/// A list of possible error reasons
#[derive(Debug)]
pub enum FtErrorKind {
    /// Any error regarding `io::Result` read operations that failed
    ReadError(io::Error),
    /// Any error regarding `io::Result` write operations that failed
    WriteError(io::Error),
    /// The file was not found
    NotFoundError,
    /// Expected a FileType::Directory, found something else
    NotADirectoryError,
    /// Expected a FileType::Symlink, found something else
    NotASymlinkError,
}

use FtErrorKind::*;

impl error::Error for FtError {
    // Should this return Option<&io::Error> instead? nope, let's keep it like this
    // while there's no logistic link error...
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ReadError(io_source) | WriteError(io_source) => Some(io_source),
            _ => None,
        }
    }
}

impl fmt::Display for FtError {
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

// impl From<io::Error> for FtError {
//     fn from(err: io::Error) -> Self {
//         FtError::IoError(err)
//     }
// }
