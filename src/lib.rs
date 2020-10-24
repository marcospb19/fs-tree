mod error;
mod groups;
mod lexer;
mod parser;

pub use error::*;
pub use groups::*;
use std::ops;

pub(crate) type Range = ops::Range<usize>;
