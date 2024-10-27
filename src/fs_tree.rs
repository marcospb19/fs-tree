//! Implementation of [`FsTree`].

use std::{
    collections::BTreeMap,
    ffi::OsStr,
    io, mem,
    ops::Index,
    path::{Path, PathBuf},
};

use file_type_enum::FileType;

use crate::{
    fs,
    iter::{Iter, NodesIter, PathsIter},
    utils, Error, Result,
};

/// The children [Trie](https://en.wikipedia.org/wiki/Trie) type alias.
pub type TrieMap = BTreeMap<PathBuf, FsTree>;

/// A filesystem tree recursive type.
///
/// # Iterators:
///
/// See the [iterator module documentation](crate::iter).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FsTree {
    /// A regular file.
    Regular,
    /// A directory, might have children `FsTree`s inside.
    Directory(TrieMap),
    /// Symbolic link, and it's target path (the link might be broken).
    Symlink(PathBuf),
}

impl FsTree {
    /// Creates an empty directory node.
    ///
    /// This is an alias to `FsTree::Directory(Default::default())`.
    ///
    /// ```
    /// use fs_tree::{FsTree, TrieMap};
    ///
    /// let result = FsTree::new_dir();
    /// let expected = FsTree::Directory(TrieMap::new());
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn new_dir() -> Self {
        Self::Directory(TrieMap::new())
    }

    /// Calculate the length by counting the leafs.
    pub fn len_leafs(&self) -> usize {
        if let Some(children) = self.children() {
            children.values().map(Self::len_leafs).sum::<usize>()
        } else if self.is_leaf() {
            1
        } else {
            0
        }
    }

    /// Calculate the length by counting all tree nodes, including the root.
    pub fn len_all(&self) -> usize {
        if let Some(children) = self.children() {
            children.values().map(Self::len_leafs).sum::<usize>()
        } else {
            1
        }
    }

    /// Construct a `FsTree` by reading from `path`, follows symlinks.
    ///
    /// If you want symlink-awareness, check [`symlink_read_at`].
    ///
    /// # Errors:
    ///
    /// - If any IO error occurs.
    /// - If any file has an unexpected file type.
    ///
    /// [`symlink_read_at`]: FsTree::read_at
    pub fn read_at(path: impl AsRef<Path>) -> Result<Self> {
        Self::__read_at(path.as_ref(), true)
    }

    /// Construct a `FsTree` by reading from `path`.
    ///
    /// If you don't want symlink-awareness, check [`read_at`].
    ///
    /// # Errors:
    ///
    /// - If any IO error occurs.
    /// - If any file has an unexpected file type.
    ///
    /// [`read_at`]: FsTree::symlink_read_at
    pub fn symlink_read_at(path: impl AsRef<Path>) -> Result<Self> {
        Self::__read_at(path.as_ref(), false)
    }

    fn __read_at(path: &Path, follow_symlinks: bool) -> Result<Self> {
        let get_file_type = if follow_symlinks {
            FileType::read_at
        } else {
            FileType::symlink_read_at
        };

        match get_file_type(path)? {
            FileType::Regular => Ok(Self::Regular),
            FileType::Directory => {
                let mut children = TrieMap::new();

                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    let entry_path = entry.path();

                    let node = Self::__read_at(&entry_path, follow_symlinks)?;

                    let stripped_file_path = entry_path
                        .strip_prefix(path)
                        .expect("Failed to strip prefix, expected to always succeed in Linux");

                    children.insert(stripped_file_path.into(), node);
                }

                Ok(Self::Directory(children))
            },
            FileType::Symlink => {
                let target_path = utils::follow_symlink(path)?;
                Ok(Self::Symlink(target_path))
            },
            other_type => {
                Err(Error::UnexpectedFileTypeError(
                    other_type,
                    path.to_path_buf(),
                ))
            },
        }
    }

    /// Construct a structural copy of this `FsTree` by reading files at the given path.
    ///
    /// In other words, the returned tree is formed of all paths in `self` that are also found in
    /// the given `path` (intersection), missing files are skipped and types might differ.
    ///
    /// This function can be useful if you need to load a subtree from a huge folder and cannot
    /// afford to load the whole folder, or if you just want to filter out every node outside of the
    /// specified structure.
    ///
    /// This function will make at maximum `self.len()` syscalls.
    ///
    /// If you don't want symlink-awareness, check [`FsTree::symlink_read_structure_at`].
    ///
    /// # Examples:
    ///
    /// ```no_run
    /// use fs_tree::FsTree;
    ///
    /// fn dynamically_load_structure() -> FsTree {
    /// #    "
    ///     ...
    /// #    "; todo!();
    /// }
    ///
    /// let structure = dynamically_load_structure();
    ///
    /// let new_tree = structure.read_structure_at("path_here").unwrap();
    ///
    /// // It is guaranteed that every path in here is present in `structure`
    /// for path in new_tree.paths() {
    ///     assert!(structure.get(path).is_some());
    /// }
    /// ```
    ///
    /// # Errors:
    ///
    /// - If an IO error happens, except [`io::ErrorKind::NotFound`]
    ///
    /// [`io::ErrorKind::NotFound`]: std::io::ErrorKind::NotFound
    pub fn read_structure_at(&self, path: impl AsRef<Path>) -> Result<Self> {
        self.__read_structure_at(path.as_ref(), true)
    }

    /// Construct a structural copy of this `FsTree` by reading files at the given path.
    ///
    /// In other words, the returned tree is formed of all paths in `self` that are also found in
    /// the given `path` (intersection), missing files are skipped and types might differ.
    ///
    /// This function can be useful if you need to load a subtree from a huge folder and cannot
    /// afford to load the whole folder, or if you just want to filter out every node outside of the
    /// specified structure.
    ///
    /// This function will make at maximum `self.len()` syscalls.
    ///
    /// If you don't want symlink-awareness, check [`FsTree::read_structure_at`].
    ///
    /// # Examples:
    ///
    /// ```no_run
    /// use fs_tree::FsTree;
    ///
    /// fn dynamically_load_structure() -> FsTree {
    /// #    "
    ///     ...
    /// #    "; todo!();
    /// }
    ///
    /// let structure = dynamically_load_structure();
    ///
    /// let new_tree = structure.symlink_read_structure_at("path_here").unwrap();
    ///
    /// // It is guaranteed that every path in here is present in `structure`
    /// for path in new_tree.paths() {
    ///     assert!(structure.get(path).is_some());
    /// }
    /// ```
    ///
    /// # Errors:
    ///
    /// - If an IO error happens, except [`io::ErrorKind::NotFound`]
    ///
    /// [`io::ErrorKind::NotFound`]: std::io::ErrorKind::NotFound
    pub fn symlink_read_structure_at(&self, path: impl AsRef<Path>) -> Result<Self> {
        self.__read_structure_at(path.as_ref(), false)
    }

    fn __read_structure_at(&self, folder: &Path, follow_symlinks: bool) -> Result<Self> {
        let mut new_tree = FsTree::new_dir();

        for relative_path in self.paths() {
            // TODO: optimize this, instead of creating a PathBuf for each path,
            // it's possible to use one mutable buffer with push + pop
            let path = folder.join(&relative_path);

            let get_file_type = if follow_symlinks {
                FileType::read_at
            } else {
                FileType::symlink_read_at
            };

            let file_type = match get_file_type(&path) {
                Ok(file_type) => file_type,
                Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
                Err(err) => return Err(err.into()),
            };

            let node = match file_type {
                FileType::Regular => Self::Regular,
                FileType::Directory => Self::new_dir(),
                FileType::Symlink => {
                    let target_path = utils::follow_symlink(&path)?;
                    Self::Symlink(target_path)
                },
                _ => continue,
            };

            new_tree.insert(relative_path, node);
        }

        Ok(new_tree)
    }

    /// Construct a `FsTree` from path pieces.
    ///
    /// Returns `None` if the input is empty.
    ///
    /// Returned value can correspond to a regular file or directory, but not a symlink.
    ///
    /// # Warning
    ///
    /// The last piece is always a file, so inputs ending with `/`, like `Path::new("example/")` are
    /// **NOT** parsed as directories.
    ///
    /// For my usage cases it's OK, but open an issue if you think otherwise 👍.
    ///
    /// # Examples:
    ///
    /// ```
    /// use fs_tree::{FsTree, tree};
    ///
    /// let result = FsTree::from_path_text("a/b/c");
    ///
    /// let expected = tree! {
    ///     a: {
    ///         b: {
    ///             c
    ///         }
    ///     }
    /// };
    ///
    /// // The expected tree
    /// assert_eq!(result, expected);
    ///
    /// // Nodes are nested
    /// assert!(result.is_dir());
    /// assert!(result["a"].is_dir());
    /// assert!(result["a"]["b"].is_dir());
    /// assert!(result["a"]["b"]["c"].is_regular());
    /// ```
    pub fn from_path_text(path: impl AsRef<Path>) -> Self {
        Self::from_path_pieces(path.as_ref().iter())
    }

    /// Generic iterator version of [`from_path_text`](FsTree::from_path_text).
    pub fn from_path_pieces<I, P>(path_iter: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<PathBuf>,
    {
        let mut path_iter = path_iter.into_iter();

        if let Some(popped_piece) = path_iter.next() {
            let child = (popped_piece.into(), Self::from_path_pieces(path_iter));
            Self::Directory(TrieMap::from([child]))
        } else {
            Self::Regular
        }
    }

    /// Creates an iterator that yields `(&FsTree, PathBuf)`.
    ///
    /// See iterator docs at the [`iter` module documentation](crate::iter).
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    /// Creates an iterator that yields `&FsTree`.
    ///
    /// See iterator docs at the [`iter` module documentation](crate::iter).
    pub fn nodes(&self) -> NodesIter {
        NodesIter::new(self)
    }

    /// Creates an iterator that yields `PathBuf`.
    ///
    /// See iterator docs at the [`iter` module documentation](crate::iter).
    pub fn paths(&self) -> PathsIter {
        PathsIter::new(self)
    }

    /// Returns `true` if `self` type matches `other` type.
    pub fn is_same_type_as(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }

    /// Returns `Ok(true)` if all nodes exist in the filesystem.
    ///
    /// # Errors:
    ///
    /// Similar to how [`Path::try_exists`] works, this function returns `false` if any IO error
    /// occurred when checking [`std::fs::symlink_metadata`] (except [`io::ErrorKind::NotFound`]).
    pub fn try_exists(&mut self) -> io::Result<bool> {
        for path in self.paths() {
            match fs::symlink_metadata(path) {
                Ok(_) => continue,
                Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
                Err(error) => return Err(error),
            }
        }

        Ok(true)
    }

    /// Merge two trees.
    ///
    /// When conflicts happen, entries from `self` are kept, and the `other`'s are discarded.
    pub fn merge(self, other: Self) -> Self {
        // let's merge the right (consuming) onto the left (mutating)
        let mut left = self;
        let right = other;

        match (&mut left, right) {
            // both a directory at the same path, try merging
            (FsTree::Directory(left_children), FsTree::Directory(right_children)) => {
                for (path, right_node) in right_children {
                    // if right node exists, remove, merge and re-add, otherwise, just add it
                    if let Some(left_node) = left_children.remove(&path) {
                        let new_node = left_node.merge(right_node);
                        left_children.insert(path, new_node);
                    } else {
                        left_children.insert(path, right_node);
                    }
                }
            },
            (_, _) => { /* conflict, but nothing to do, don't mutate left side */ },
        }

        left
    }

    /// Checks for conflicts in case the two trees would be merged.
    ///
    /// Also see [`Self::merge`].
    pub fn conflicts_with(&self, other: &Self) -> bool {
        let mut left = self;
        let right = other;

        match (&mut left, right) {
            (FsTree::Directory(left_children), FsTree::Directory(right_children)) => {
                for (path, right_node) in right_children {
                    if let Some(left_node) = left_children.get(path.as_path()) {
                        if left_node.conflicts_with(right_node) {
                            return true;
                        }
                    }
                }
            },
            (_, _) => return true,
        }

        false
    }

    /// Reference to children if `self.is_directory()`.
    pub fn children(&self) -> Option<&TrieMap> {
        match &self {
            Self::Directory(children) => Some(children),
            _ => None,
        }
    }

    /// Mutable reference to children if `self.is_directory()`.
    pub fn children_mut(&mut self) -> Option<&mut TrieMap> {
        match self {
            Self::Directory(children) => Some(children),
            _ => None,
        }
    }

    /// Reference to target path, if `self.is_symlink()`.
    pub fn target(&self) -> Option<&Path> {
        match &self {
            Self::Symlink(target_path) => Some(target_path),
            _ => None,
        }
    }

    /// Mutable reference to target path, if `self.is_symlink()`.
    pub fn target_mut(&mut self) -> Option<&mut PathBuf> {
        match self {
            Self::Symlink(target_path) => Some(target_path),
            _ => None,
        }
    }

    // /// Apply a closure for each direct child of this FsTree.
    // ///
    // /// Only 1 level deep.
    // pub fn apply_to_children0(&mut self, f: impl FnMut(&mut Self)) {
    //     if let Some(children) = self.children_mut() {
    //         children.iter_mut().for_each(f);
    //     }
    // }

    // /// Apply a closure to all direct and indirect descendants inside of this structure.
    // ///
    // /// Calls recursively for all levels.
    // pub fn apply_to_all_children1(&mut self, f: impl FnMut(&mut Self) + Copy) {
    //     if let Some(children) = self.children_mut() {
    //         children
    //             .iter_mut()
    //             .for_each(|x| x.apply_to_all_children1(f));
    //         children.iter_mut().for_each(f);
    //     }
    // }

    // /// Apply a closure to all direct and indirect descendants inside (including root).
    // ///
    // /// Calls recursively for all levels.
    // pub fn apply_to_all(&mut self, mut f: impl FnMut(&mut Self) + Copy) {
    //     f(self);
    //     if let Some(children) = self.children_mut() {
    //         for child in children.iter_mut() {
    //             child.apply_to_all(f);
    //         }
    //     }
    // }

    /// Returns `true` if `self` is a leaf node.
    ///
    /// A leaf node might be of any type, including directory, however, a
    /// non-leaf node is always a directory.
    pub fn is_leaf(&self) -> bool {
        match self {
            Self::Regular | Self::Symlink(_) => true,
            Self::Directory(children) => children.is_empty(),
        }
    }

    /// The variant string, useful for showing to user.
    pub fn variant_str(&self) -> &'static str {
        match self {
            Self::Regular => "regular file",
            Self::Directory(_) => "directory",
            Self::Symlink(_) => "symlink",
        }
    }

    /// Returns `true` if self matches the [`FsTree::Regular`] variant.
    pub fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }

    /// Returns `true` if self matches the [`FsTree::Directory`] variant.
    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Directory(_))
    }

    /// Returns `true` if self matches the [`FsTree::Symlink`] variant.
    pub fn is_symlink(&self) -> bool {
        matches!(self, Self::Symlink(_))
    }

    // /// Generate a diff from two different trees.
    // pub fn diff(&self, other: &Self) {
    //     if !self.has_same_type_as(other) {
    //         println!("Types differ! ");
    //     }

    //     let (self_children, other_children) = match (&self.file_type, &other.file_type) {
    //         (Self::Directory(self_children), Self::Directory(other_children)) => {
    //             (self_children, other_children)
    //         },
    //         _ => panic!(),
    //     };

    //     let mut lookup = self_children
    //         .iter()
    //         .map(|x| (&x.path, x))
    //         .collect::<HashMap<&PathBuf, &FsTree>>();

    //     for other_child in other_children {
    //         if let Some(self_child) = lookup.remove(&other_child.path) {
    //             if self_child.has_same_type_as(other_child) {
    //                 if self_child.is_dir() {
    //                     self_child.diff(other_child);
    //                 }
    //             } else {
    //                 println!(
    //                     "File {:?} is a {} while file {:?} is a {}",
    //                     self_child.path,
    //                     self_child.file_type.file_type_display(),
    //                     other_child.path,
    //                     other_child.file_type.file_type_display(),
    //                 );
    //             }
    //         } else {
    //             let path = &other_child.path;
    //             println!(
    //                 "2Only in {:?}: {:?}",
    //                 path.parent().unwrap(),
    //                 path.file_name().unwrap()
    //             );
    //         }
    //     }

    //     for child_left in lookup.values() {
    //         let path = &child_left.path;
    //         println!(
    //             "1Only in {:?}: {:?}",
    //             path.parent().unwrap(),
    //             path.file_name().unwrap()
    //         );
    //     }
    // }

    /// Write the tree structure in the path.
    ///
    /// # Errors:
    ///
    /// - If provided folder doesn't exist, or is not a directory.
    /// - If any other IO error occurs.
    pub fn write_at(&self, folder: impl AsRef<Path>) -> Result<()> {
        let folder = folder.as_ref();

        #[cfg(feature = "fs-err")]
        let symlink_function = fs_err::os::unix::fs::symlink;
        #[cfg(not(feature = "fs-err"))]
        let symlink_function = std::os::unix::fs::symlink;

        for (node, path) in self.iter().skip(1) {
            let path = folder.join(&path);

            match &node {
                Self::Regular => {
                    fs::File::create(path)?;
                },
                Self::Directory(_) => {
                    fs::create_dir(path)?;
                },
                Self::Symlink(target) => {
                    symlink_function(target, path)?;
                },
            }
        }

        Ok(())
    }

    /// Returns a reference to the node at the path, if any.
    ///
    /// # Errors:
    ///
    /// - Returns `None` if there is no node at the given path.
    ///
    /// # Examples:
    ///
    /// ```
    /// use fs_tree::FsTree;
    ///
    /// let root = FsTree::from_path_text("a/b/c");
    ///
    /// // Indexing is relative from `root`, so `root` cannot be indexed.
    /// assert_eq!(root, FsTree::from_path_text("a/b/c"));
    /// assert_eq!(root["a"], FsTree::from_path_text("b/c"));
    /// assert_eq!(root["a/b"], FsTree::from_path_text("c"));
    /// assert_eq!(root["a"]["b"], FsTree::from_path_text("c"));
    /// assert_eq!(root["a/b/c"], FsTree::Regular);
    /// assert_eq!(root["a/b"]["c"], FsTree::Regular);
    /// assert_eq!(root["a"]["b/c"], FsTree::Regular);
    /// assert_eq!(root["a"]["b"]["c"], FsTree::Regular);
    /// ```
    pub fn get(&self, path: impl AsRef<Path>) -> Option<&Self> {
        let path = path.as_ref();

        // Split first piece from the rest
        let (popped, path_rest) = {
            let mut iter = path.iter();
            let popped: Option<&Path> = iter.next().map(OsStr::as_ref);
            (popped, iter.as_path())
        };

        // If path ended, we reached the desired node
        let Some(popped) = popped else {
            return Some(self);
        };

        // Corner case: if `.`, ignore it and call again with the rest
        if popped == Path::new(".") {
            return self.get(path_rest);
        }

        self.children()?
            .get(popped)
            .and_then(|child| child.get(path_rest))
    }

    /// Returns a mutable reference to the node at the path, if any.
    ///
    /// This is the mutable version of [`FsTree::get`].
    pub fn get_mut(&mut self, path: impl AsRef<Path>) -> Option<&mut Self> {
        let path = path.as_ref();

        // Split first piece from the rest
        let (popped, path_rest) = {
            let mut iter = path.iter();
            let popped: Option<&Path> = iter.next().map(OsStr::as_ref);
            (popped, iter.as_path())
        };

        // If path ended, we reached the desired node
        let Some(popped) = popped else {
            return Some(self);
        };

        // Corner case: if `.`, ignore it and call again with the rest
        if popped == Path::new(".") {
            return self.get_mut(path_rest);
        }

        self.children_mut()?
            .get_mut(popped)
            .and_then(|child| child.get_mut(path_rest))
    }

    /// Inserts a node at the given path.
    ///
    /// # Panics:
    ///
    /// - If there are no directories up to the path node in order to insert it.
    /// - If path is empty.
    pub fn insert(&mut self, path: impl AsRef<Path>, node: Self) {
        use FsTree::*;

        let mut iter = path.as_ref().iter();

        let Some(node_name) = iter.next_back().map(Path::new) else {
            *self = node;
            return;
        };

        let mut tree = self;

        // Traverse tree
        for next in iter {
            // Give a better error message than the one below
            if !tree.is_dir() {
                panic!(
                    "Failed to insert node, while traversing, one of the parent directories \
                    ({next:?}) isn't a directory, but a {}",
                    tree.variant_str()
                );
            }

            tree = if let Some(tree) = tree.get_mut(next) {
                tree
            } else {
                panic!("Failed to insert node, parent directory {next:?} doesn't exist");
            };
        }

        match tree {
            Regular | Symlink(_) => {
                panic!(
                    "Failed to insert node, parent directory is not a directory, but a {}",
                    tree.variant_str(),
                );
            },
            Directory(children) => {
                children.insert(node_name.into(), node);
            },
        }
    }
}

#[cfg(feature = "libc-file-type")]
impl FsTree {
    /// Returns the file type equivalent [`libc::mode_t`] value.
    pub fn as_mode_t(&self) -> libc::mode_t {
        match self {
            Self::Regular => libc::S_IFREG,
            Self::Directory(_) => libc::S_IFDIR,
            Self::Symlink(_) => libc::S_IFCHR,
        }
    }
}

impl<P> Index<P> for FsTree
where
    P: AsRef<Path>,
{
    type Output = FsTree;

    fn index(&self, path: P) -> &Self::Output {
        self.get(path.as_ref())
            .unwrap_or_else(|| panic!("no node found for path '{}'", path.as_ref().display()))
    }
}

#[cfg(test)]
mod tests {
    use std::{io, path::Path};

    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;
    use crate::tree;

    fn testdir() -> io::Result<(tempfile::TempDir, &'static Path)> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().to_path_buf().into_boxed_path();
        Ok((dir, Box::leak(path)))
    }

    // #[test]
    // fn test_diff() {
    //     let left = FsTree::from_path_text(".config/i3/file").unwrap();
    //     let right = FsTree::from_path_text(".config/i3/folder/file/oie").unwrap();
    //     left.diff(&right);
    //     panic!();
    // }

    #[test]
    fn test_insert_basic() {
        let mut tree = FsTree::new_dir();

        let paths = ["a", "a/b", "a/b/c", "a/b/c/d", "a/b/c/d/e"];
        for path in paths {
            tree.insert(path, FsTree::new_dir());
        }

        tree.insert("a/b/c/d/e/f", FsTree::Regular);

        let expected = tree! {
            a: { b: { c: { d: { e: { f } } } } }
        };

        assert_eq!(tree, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_insert_complete() {
        let result = {
            let mut tree = FsTree::new_dir();
            tree.insert("config1", FsTree::Regular);
            tree.insert("config2", FsTree::Regular);
            tree.insert("outer_dir", FsTree::new_dir());
            tree.insert("outer_dir/file1", FsTree::Regular);
            tree.insert("outer_dir/file2", FsTree::Regular);
            tree.insert("outer_dir/inner_dir", FsTree::new_dir());
            tree.insert("outer_dir/inner_dir/inner1", FsTree::Regular);
            tree.insert("outer_dir/inner_dir/inner2", FsTree::Regular);
            tree.insert("outer_dir/inner_dir/inner3", FsTree::Regular);
            tree.insert("outer_dir/inner_dir/inner_link", FsTree::Symlink("inner_target".into()));
            tree.insert("link", FsTree::Symlink("target".into()));
            tree.insert("config3", FsTree::Regular);
            tree
        };

        let expected = tree! {
            config1
            config2
            outer_dir: {
                file1
                file2
                inner_dir: {
                    inner1
                    inner2
                    inner3
                    inner_link -> inner_target
                }
            }
            link -> target
            config3
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_write_at() {
        let (_dropper, test_dir) = testdir().unwrap();

        let tree = tree! {
            a: {
                b: {
                    c
                    empty: {}
                    link -> target
                }
            }
        };

        tree.write_at(test_dir).unwrap();

        let result = FsTree::symlink_read_at(test_dir).unwrap();

        assert_eq!(result, tree);
    }

    #[test]
    fn test_get() {
        let tree = FsTree::from_path_text("a/b/c");

        assert_eq!(tree["a"], FsTree::from_path_text("b/c"));
        assert_eq!(tree["a/b"], FsTree::from_path_text("c"));
        assert_eq!(tree["a"]["b"], FsTree::from_path_text("c"));
        assert_eq!(tree["a/b/c"], FsTree::Regular);
        assert_eq!(tree["a/b"]["c"], FsTree::Regular);
        assert_eq!(tree["a"]["b/c"], FsTree::Regular);
        assert_eq!(tree["a"]["b"]["c"], FsTree::Regular);

        // Paths are relative, so empty path returns the node itself
        assert_eq!(tree[""], tree);
        assert_eq!(tree[""], tree[""]);

        // "."s are ignored
        assert_eq!(tree["."], tree[""]);
        assert_eq!(tree["././"], tree["."]);
        assert_eq!(tree["././."], tree);
        assert_eq!(tree["./a/."]["././b/./."], FsTree::from_path_text("c"));
        assert_eq!(tree["./a/./b"]["c/."], FsTree::Regular);
    }

    // #[test]
    // fn test_simple_merge() {
    //     let left = FsTree::from_path_text(".config/i3/file");
    //     let right = FsTree::from_path_text(".config/i3/folder/file");
    //     let result = left.try_merge(right);

    //     let expected = tree! {
    //         ".config": {
    //             i3: {
    //                 file
    //                 folder: {
    //                     file
    //                 }
    //             }
    //         }
    //     };

    //     assert_eq!(result, Some(expected));
    // }

    #[test]
    fn test_partial_eq_fails() {
        let left = FsTree::from_path_text(".config/i3/a");
        let right = FsTree::from_path_text(".config/i3/b");

        assert_ne!(left, right);
    }
}
