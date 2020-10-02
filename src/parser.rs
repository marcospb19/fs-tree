use crate::{lexer::Token, Value};
use std::collections::BTreeMap;

// pub fn parse_text() -> Groups {
//     Groups::new()
// }

// pub enum Value {
//     // Group(String),
//     // File(String),
//     // Directory(String, Vec<Value>),
//     // SymLink(String, String),
// }

pub fn parse_tokens(tokens: Vec<(Token, std::ops::Range<usize>)>) -> BTreeMap<String, Vec<Value>> {
    let mut groups = BTreeMap::<String, Vec<Value>>::new();

    // There's always a "main" group, it's filled when no other group is declared
    groups.insert(String::from("main"), vec![]);

    // Check if any error occurred
    if let Token::Error = tokens[0].0 {
        unimplemented!();
    }

    groups
}
