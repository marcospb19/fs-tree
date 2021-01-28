use std::collections::BTreeSet;

#[derive(Debug, Default, Hash, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Flags {
    pub direct_flags: BTreeSet<String>,
    pub group_flags: BTreeSet<String>,
    pub ancestor_flags: BTreeSet<String>,
}

impl Flags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_direct_flag(&mut self, name: impl AsRef<str>) -> bool {
        self.direct_flags.insert(name.as_ref().to_string())
    }

    pub fn add_group_flag(&mut self, name: impl AsRef<str>) -> bool {
        self.group_flags.insert(name.as_ref().to_string())
    }

    pub fn add_ancestor_flag(&mut self, name: impl AsRef<str>) -> bool {
        self.ancestor_flags.insert(name.as_ref().to_string())
    }

    pub fn inherit_from(&mut self, parent: &Self) {
        // Set `a` receives all elements from set `b`
        let merge_sets = |a: &mut BTreeSet<String>, b: &BTreeSet<String>| {
            b.iter().for_each(|x| {
                a.insert(x.clone());
            });
        };

        // Direct flags, from parents, become ancestor_flags
        merge_sets(&mut self.ancestor_flags, &parent.direct_flags);

        // Replicated
        merge_sets(&mut self.group_flags, &parent.group_flags);
        merge_sets(&mut self.ancestor_flags, &parent.ancestor_flags);
    }
}
