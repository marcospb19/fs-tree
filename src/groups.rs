use crate::{lexer::run_lexer, parser::parse_tokens, GroupsMap};

#[derive(Debug, Hash, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Groups {
    pub map: GroupsMap,
}

impl Groups {
    pub fn from_text(text: &str) -> Self {
        let tokens = run_lexer(text);
        let map = parse_tokens(tokens);

        Groups { map }
    }
}
