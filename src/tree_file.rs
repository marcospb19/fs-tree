use crate::{lexer, parser};

use std::{
    collections::{btree_map, BTreeMap},
    ops,
};

// Remove this default bro lol
#[derive(Debug, Default)]
pub struct Groups {
    pub owned_groups: BTreeMap<String, Vec<Value>>,
}

impl Groups {
    pub fn is_empty(&self) -> bool {
        self.owned_groups.is_empty()
    }

    pub fn len(&self) -> usize {
        self.owned_groups.len()
    }

    pub fn iter(&self) -> btree_map::Iter<String, Vec<Value>> {
        self.owned_groups.iter()
    }

    pub fn into_iter(self) -> btree_map::IntoIter<String, Vec<Value>> {
        self.owned_groups.into_iter()
    }

    pub fn from_text(text: impl AsRef<str>) -> Self {
        let tokens = lexer::text_as_tokens(text);
        let groups: BTreeMap<String, Vec<Value>> = parser::parse_tokens(tokens);

        Groups {
            owned_groups: groups,
        }

        // let text = text.as_ref();
        // parser::parse_tokens(text)
    }
}

impl ops::Index<&str> for Groups {
    type Output = Vec<Value>;

    fn index(&self, arg: &str) -> &Self::Output {
        self.owned_groups.index(arg)
    }
}

#[derive(Debug)]
pub struct Group {
    pub values: Vec<Value>,
}

#[derive(Debug)]
pub enum Value {
    File(String),
    Directory(String, Vec<Value>),
}
