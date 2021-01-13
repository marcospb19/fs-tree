// #[derive(Debug, Hash, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[derive(Debug, Clone, Default)]

pub struct Flags {
    pub inner: Vec<Flag>,
}

impl Flags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(vec: Vec<Flag>) -> Self {
        Self { inner: vec }
    }

    pub fn add_flag(&mut self, flag: Flag) {
        self.inner.push(flag);
    }
}

#[derive(Debug, Clone)]
pub struct Flag {
    pub name: String,
    pub flag_type: FlagType,
}

impl Flag {
    pub fn new(name: impl AsRef<str>, flag_type: FlagType) -> Self {
        let name = name.as_ref().to_string();
        Self { name, flag_type }
    }
}

#[derive(Debug, Clone)]
pub enum FlagType {
    Direct,
    ParentInherited,
    GroupInherited,
}
