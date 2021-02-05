mod error;
mod flags;
mod groups;
mod lexer;
mod parser;

use std::collections::BTreeMap;

pub use error::TreeFileError;
pub use groups::Groups;
pub use lexer::LexToken;

use crate::flags::Flags;
pub type File = file_tree::File<Flags>;
pub use file_tree::FileType;
//
pub type GroupsMap = BTreeMap<String, Vec<File>>;
