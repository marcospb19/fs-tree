use crate::{lexer::run_lexer, parser::parse_tokens, GroupsMap};
use std::{path::PathBuf, process};

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

        Groups {
            map,
            info: GroupsInfo {
                file_path: None,
                groups_order,
            },
        }
    }
}

#[derive(Debug)]
pub struct GroupsInfo {
    file_path: Option<PathBuf>,
    groups_order: Vec<String>,
}
