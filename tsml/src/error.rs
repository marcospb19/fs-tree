use std::{error, fmt, io};

#[derive(Debug)]
pub enum TsmlError {
    LexerError,
    ParserError(crate::parser::ParserError),
    IoError(io::Error),
    Other,
}

pub type TsmlResult<T> = Result<T, TsmlError>;

impl fmt::Display for TsmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tsml: ")?;
        match self {
            TsmlError::LexerError => write!(f, "lexer error"),
            TsmlError::ParserError(..) => write!(f, "parser error"),
            TsmlError::IoError(err) => write!(f, "Io error: {}", err),
            TsmlError::Other => write!(f, "other error"),
        }
    }
}

impl error::Error for TsmlError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<crate::parser::ParserError> for TsmlError {
    fn from(source: crate::parser::ParserError) -> Self {
        TsmlError::ParserError(source)
    }
}

impl From<io::Error> for TsmlError {
    fn from(source: io::Error) -> Self {
        TsmlError::IoError(source)
    }
}
