mod error;
mod flags;
mod groups;
mod lexer;
mod parser;

pub use error::TreeFileError;
pub use groups::Groups;
pub use lexer::LexToken;

use std::collections::BTreeMap;

use crate::flags::Flags;
pub(crate) type File = file_structure::File<Flags>;
pub(crate) use file_structure::FileType;
//
pub(crate) type GroupsMap = BTreeMap<String, Vec<File>>;
