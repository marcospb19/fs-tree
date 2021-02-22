use std::collections::BTreeSet;

#[derive(Debug, Default, Hash, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Tags {
    pub direct_tags: BTreeSet<String>,
    pub group_tags: BTreeSet<String>,
    pub ancestor_tags: BTreeSet<String>,
}

impl Tags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_direct_tag(&mut self, name: impl AsRef<str>) -> bool {
        self.direct_tags.insert(name.as_ref().to_string())
    }

    pub fn add_group_tag(&mut self, name: impl AsRef<str>) -> bool {
        self.group_tags.insert(name.as_ref().to_string())
    }

    pub fn add_ancestor_tag(&mut self, name: impl AsRef<str>) -> bool {
        self.ancestor_tags.insert(name.as_ref().to_string())
    }

    pub fn inherit_from(&mut self, parent: &Self) {
        // Set `a` receives all elements from set `b`
        let merge_sets = |a: &mut BTreeSet<String>, b: &BTreeSet<String>| {
            b.iter().for_each(|x| {
                a.insert(x.clone());
            });
        };

        // Direct tags, from parents, become ancestor_tags
        merge_sets(&mut self.ancestor_tags, &parent.direct_tags);

        // Replicated
        merge_sets(&mut self.group_tags, &parent.group_tags);
        merge_sets(&mut self.ancestor_tags, &parent.ancestor_tags);
    }
}
