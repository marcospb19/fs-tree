use std::{
    collections::{btree_map, BTreeMap},
    ops,
};

#[derive(Debug)]
pub struct Groups {
    pub owned_groups: BTreeMap<String, Group>,
}

impl Groups {
    pub fn is_empty(&self) -> bool {
        self.owned_groups.is_empty()
    }

    pub fn len(&self) -> usize {
        self.owned_groups.len()
    }

    pub fn iter(&self) -> btree_map::Iter<String, Group> {
        self.owned_groups.iter()
    }

    pub fn into_iter(self) -> btree_map::IntoIter<String, Group> {
        self.owned_groups.into_iter()
    }
}

impl ops::Index<&str> for Groups {
    type Output = Group;

    fn index(&self, arg: &str) -> &Self::Output {
        &self.owned_groups[arg]
    }
}

#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub values: Vec<Value>,
}

#[derive(Debug)]
pub enum Value {
    File(String),
    Directory(String, Vec<Value>),
}
