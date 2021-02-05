use std::{
    fs, io,
    path::{Path, PathBuf},
    process,
};

use crate::{lexer::run_lexer, parser::parse_tokens, GroupsMap};

#[derive(Debug)]
// #[derive(Debug, Hash, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Groups {
    pub map: GroupsMap,
    pub info: GroupsInfo,
}

impl Groups {
    pub fn from_text(text: &str) -> Self {
        let tokens = run_lexer(text);
        let (map, groups_order) = parse_tokens(tokens, text).unwrap_or_else(|err| {
            eprintln!("Error: '{}'", err);
            process::exit(1);
        });

        Groups { map, info: GroupsInfo { file_path: None, groups_order } }
    }

    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let text = fs::read_to_string(path.as_ref())?;
        let mut result = Self::from_text(&text);
        result.info.file_path = Some(path.as_ref().to_path_buf());
        Ok(result)
    }
}

#[derive(Debug)]
pub struct GroupsInfo {
    file_path: Option<PathBuf>,
    groups_order: Vec<String>,
}
