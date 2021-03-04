use std::{error, fmt, io, path::PathBuf};

use crate::parser::{ParserErrorKind, TokenPosition};

#[derive(Debug)]
pub enum TsmlError {
    LexerError,
    ParserError(TokenPosition, ParserErrorKind),
    IoError(io::Error),
    PathWithoutName, // "we found a file that does not contain a name!"
    NonUtf8Path(PathBuf),
    Other,
}

pub type TsmlResult<T> = Result<T, TsmlError>;

impl fmt::Display for TsmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tsml: ")?;
        match self {
            TsmlError::LexerError => {
                write!(f, "lexer error")
            },
            TsmlError::IoError(err) => {
                write!(f, "Io error: {}", err)
            },
            TsmlError::PathWithoutName => {
                write!(f, "found an empty (thus invalid) path")
            },
            TsmlError::NonUtf8Path(path) => {
                write!(f, "found a path name that is invalid UTF-8: {}", path.to_string_lossy())
            },
            TsmlError::Other => {
                write!(f, "other error")
            },
            TsmlError::ParserError(position, kind) => {
                write!(f, "parser error at {}:{}: ", position.line, position.column)?;
                use ParserErrorKind::*;
                eprintln!("asd");
                // todo!("ajeita isso aqui");
                match kind {
                    BracketUnclosed => {
                        write!(f, "unclosed brackets")
                    },
                    BracketUnexpectedClose => {
                        write!(f, "unexpected close brackets, what are you closing?")
                    },
                    BracketUnexpectedOpen => {
                        write!(f, "what are you trying to open there?????")
                    },
                    CommasOutsideOfBrackets => {
                        write!(f, "no commas alowed outsite of scopes")
                    },
                    MissingSymlinkTarget => {
                        write!(f, "arrow without the plim plimplimplim")
                    },
                    TagAfterTag => {
                        write!(f, "tag after tag problemo")
                    },
                }
            },
        }
    }
}

impl error::Error for TsmlError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            TsmlError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for TsmlError {
    fn from(source: io::Error) -> Self {
        TsmlError::IoError(source)
    }
}
