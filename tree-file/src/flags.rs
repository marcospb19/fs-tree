// #[derive(Debug, Hash, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[derive(Debug, Clone, Default)]

pub struct Flags {
    pub inner: Vec<Flag>,
}

impl Flags {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct Flag {
    name: String,
    kind: FlagType,
}

impl Flag {
    pub fn new(name: impl AsRef<str>, kind: FlagType) -> Self {
        let name = name.as_ref().to_string();
        Self { name, kind }
    }
}

#[derive(Debug, Clone)]
pub enum FlagType {
    Direct,
    ParentInherited,
    GroupInherited,
}
