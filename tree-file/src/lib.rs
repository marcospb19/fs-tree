mod error;
mod groups;
mod lexer;
mod parser;
// mod value;

pub use error::TreeFileError;
pub use groups::Groups;
pub use lexer::LexToken;

use std::collections::BTreeMap;

pub(crate) type File = file_structure::File<()>;
pub(crate) use file_structure::FileType;
//
pub(crate) type GroupsMap = BTreeMap<String, Vec<File>>;
