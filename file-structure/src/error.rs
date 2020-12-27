use std::{error, fmt, io, path::PathBuf};

/// The only `Result` used in the public API of this crate
pub type FsResult<T> = Result<T, FsError>;

/// The error type for this crate
#[derive(Debug)]
pub struct FsError {
    context: &'static str, // Optional aditional context for debugging purposes, can be empty
    kind: FsErrorKind,
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
impl FsError {
    pub(crate) fn new(kind: FsErrorKind, path: PathBuf, context: &'static str) -> Self {
        Self {
            context,
            kind,
            path,
        }
    }

    /// Description of the error
    pub fn context(&self) -> &'static str {
        self.context
    }

    /// Enum with all possible error variants
    pub fn kind(&self) -> &FsErrorKind {
        &self.kind
    }

    /// Path to where the error occurred
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

/// A list of possible error reasons
#[derive(Debug)]
pub enum FsErrorKind {
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

use FsErrorKind::*;

impl error::Error for FsError {
    // Should this return Option<&io::Error> instead? nope, let's keep it like this
    // while there's no logistic link error...
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ReadError(io_source) | WriteError(io_source) => Some(io_source),
            _ => None,
        }
    }
}

impl fmt::Display for FsError {
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

// impl From<io::Error> for FsError {
//     fn from(err: io::Error) -> Self {
//         FsError::IoError(err)
//     }
// }
