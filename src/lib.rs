pub mod lexer;
pub mod parser;

#[derive(Debug)]
pub struct Value {}

#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub values: Vec<Value>,
}

#[derive(Debug)]
pub struct Groups {
    pub underlying_vec: Vec<Group>,
}

// impl IntoIter for Groups {}

pub fn get_groups(text: impl AsRef<str>) -> Vec<Group> {
    let _text = text.as_ref();
    vec![]
}
