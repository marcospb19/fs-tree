mod error;
mod groups;
mod lexer;
mod parser;
mod serializer;
mod tags;

use std::collections::BTreeMap;

pub use error::TreeFileError;
pub use groups::Groups;
pub use lexer::LexToken;

use crate::tags::Tags;
pub type FileTree = file_tree::FileTree<Tags>;
//
pub type GroupsMap = BTreeMap<String, Vec<FileTree>>;

pub use serializer::groups_to_tsml;
