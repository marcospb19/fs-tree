use std::{error, fmt};

#[derive(Debug)]
pub enum TreeFileError {
    LexerError,
    ParserError,
    Other,
}

impl fmt::Display for TreeFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tree file: ")?;
        match self {
            TreeFileError::LexerError => write!(f, "lexer error."),
            TreeFileError::ParserError => write!(f, "parser error."),
            TreeFileError::Other => write!(f, "other error."),
        }
    }
}

impl error::Error for TreeFileError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
