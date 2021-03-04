use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{lexer::run_lexer, parser::parse_tokens, GroupsMap, TsmlResult};

#[derive(Debug, Clone)]
pub struct Groups {
    pub map: GroupsMap,
    pub info: GroupsInfo,
}

// Collect the start of the file to retrieve as GroupsInfo
fn get_file_header(text: &str) -> String {
    text.lines().take_while(|line| line.starts_with("//") || line.is_empty()).collect::<String>()
}

impl Groups {
    pub fn from_text(text: &str) -> TsmlResult<Self> {
        let tokens = run_lexer(text);
        parse_tokens(tokens, text).map(|(map, groups_order)| Groups {
            map,
            info: GroupsInfo { file_path: None, file_header: get_file_header(text), groups_order },
        })
    }

    pub fn from_path(path: impl AsRef<Path>) -> TsmlResult<Self> {
        let text = fs::read_to_string(path.as_ref())?;
        let mut result = Self::from_text(&text)?;
        result.info.file_path = Some(path.as_ref().to_path_buf());
        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct GroupsInfo {
    pub file_path: Option<PathBuf>,
    pub file_header: String,
    pub groups_order: Vec<String>,
}
