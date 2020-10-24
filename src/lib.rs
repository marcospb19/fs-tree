mod error;
mod groups;
mod lexer;
mod parser;
mod value;

pub use error::TreeFileError;
pub use groups::Groups;
pub use value::Value;

use std::{collections::BTreeMap, ops};

pub(crate) type Range = ops::Range<usize>;
pub(crate) type GroupsMap = BTreeMap<String, Vec<Value>>;
