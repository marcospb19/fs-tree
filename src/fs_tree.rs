//! Implementation of the filesystem tree.

use std::{
    collections::HashMap,
    ops::Index,
    path::{Path, PathBuf},
};

use file_type_enum::FileType;

use crate::{
    fs,
    iter::{FilesIter, Iter, PathsIter},
    util, Error, Result, TreeNode,
};

/// A filesystem tree recursive type.
///
/// Methods for iteration: [`.iter()`], [`.nodes()`] or [`.paths()`].
///
/// [`.iter()`]: Self::iter
/// [`.nodes()`]: Self::nodes
/// [`.paths()`]: Self::paths
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FsTree {
    /// The filename of this file.
    pub path: PathBuf,
    /// The TreeNode of this file.
    pub file_type: TreeNode,
}

impl FsTree {
    /// Creates a `FsTree::Regular` from arguments.
    pub fn new_regular(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            file_type: TreeNode::Regular,
        }
    }

    /// Creates a `FsTree::Directory` from arguments.
    pub fn new_directory(path: impl Into<PathBuf>, children: Vec<Self>) -> Self {
        Self {
            path: path.into(),
            file_type: TreeNode::Directory(children),
        }
    }

    /// Creates a `FsTree::Symlink` from arguments.
    pub fn new_symlink(path: impl Into<PathBuf>, target_path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let target_path = target_path.into();
        Self {
            path,
            file_type: TreeNode::Symlink(target_path),
        }
    }

    /// Collects a `Vec` of `FsTree` from `path` that is a directory.
    pub fn collect_from_directory(path: impl AsRef<Path>) -> Result<Vec<Self>> {
        Self::__collect_from_directory(path.as_ref(), true)
    }

    /// Collects a `Vec` of `FsTree` from `path` that is a directory, entries can be symlinks.
    pub fn collect_from_directory_symlink(path: impl AsRef<Path>) -> Result<Vec<Self>> {
        Self::__collect_from_directory(path.as_ref(), false)
    }

    /// Builds a `FsTree` from `path`, follows symlinks.
    ///
    /// Similar to `from_path_symlink`.
    ///
    /// If file at `path` is a regular file, will return a `FsTree::Regular`.
    /// If file at `path` is a directory file, `FsTree::Directory` (with .children).
    ///
    /// # Errors:
    /// - If `Io::Error` from `fs::metadata(path)`
    /// - If it is a directory, and `Io::Error` from `fs::read_dir(path)` iterator usage
    /// - If [unexpected file type] at `path`
    ///
    /// This function traverses symlinks until final destination, and then reads it, so it can never
    /// return `Ok(FsTree::Symlink { .. ]})`, if you wish otherwise, use
    /// `FsTree::from_path_symlink` instead.
    ///
    /// [unexpected file type]: docs.rs/file_type_enum
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        Self::__from_path(path.as_ref(), true)
    }

    /// Builds a `FsTree` from `path`, follows symlinks.
    ///
    /// Similar to `from_path_symlink`.
    ///
    /// If file at `path` is a regular file, will return a `FsTree::Regular`.
    /// If file at `path` is a directory file, `FsTree::Directory` (with `children` field).
    /// If file at `path` is a symlink file, `FsTree::Symlink` (with `target_path` field).
    ///
    /// # Errors:
    /// - If `Io::Error` from `fs::metadata(path)`
    /// - If it is a directory, and `Io::Error` from `fs::read_dir(path)` iterator usage
    /// - If it is a symlink, and `Io::Error` from `fs::read_link(path)`
    /// - If [unexpected file type] at `path`
    ///
    /// If you wish to traverse symlinks until final destination, instead, use
    /// `FsTree::from_path`.
    ///
    /// [unexpected file type]: docs.rs/file_type_enum
    pub fn from_path_symlink(path: impl AsRef<Path>) -> Result<Self> {
        Self::__from_path(path.as_ref(), false)
    }

    /// Splits `Path` pieces into a `FsTree`.
    ///
    /// Returns `None` if the string is empty.
    ///
    /// Can only build Regular and Directory, not symlink.
    ///
    /// Example:
    ///
    /// ```
    /// use fs_tree::{FsTree, tree};
    ///
    /// let result = FsTree::from_path_text("dir/inner/file");
    ///
    /// let expected = tree! {
    ///     dir: {
    ///         inner: {
    ///             file
    ///         }
    ///     }
    /// };
    ///
    /// assert_eq!(result, Some(expected));
    /// ```
    pub fn from_path_text(path: impl AsRef<Path>) -> Option<Self> {
        Self::from_path_pieces(path.as_ref().iter())
    }

    /// Generic version of `FsTree::from_path_text`.
    ///
    /// Returns `None` if path is empty.
    pub fn from_path_pieces<I, P>(path_iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut path_iter = path_iter.into_iter();

        let popped_piece = path_iter.next()?;

        if let Some(subtree) = Self::from_path_pieces(path_iter) {
            Self::new_directory(popped_piece.as_ref(), vec![subtree])
        } else {
            Self::new_regular(popped_piece.as_ref())
        }
        .into()
    }

    fn __collect_from_directory(folder_path: &Path, follow_symlinks: bool) -> Result<Vec<Self>> {
        if !FileType::from_path(folder_path)?.is_directory() {
            return Err(Error::NotADirectoryError(folder_path.to_path_buf()));
        }

        let mut children = vec![];

        for entry in fs::read_dir(folder_path)? {
            let entry = entry?;
            let entry_path = entry.path();

            let mut file = Self::__from_path(&entry_path, follow_symlinks)?;

            let stripped_file_path = entry_path.strip_prefix(folder_path).expect(
                "Failed to strip prefix that was assumed to always succeed, this \
                 is an error in the library `fs-tree`, please open an issue",
            );

            file.path = stripped_file_path.into();
            children.push(file);
        }

        Ok(children)
    }

    fn __from_path(path: &Path, follow_symlinks: bool) -> Result<Self> {
        let get_file_type = if follow_symlinks {
            FileType::from_path
        } else {
            FileType::from_symlink_path
        };

        match get_file_type(path)? {
            FileType::Regular => Ok(Self::new_regular(path)),
            FileType::Directory => {
                let children = Self::__collect_from_directory(path, follow_symlinks)?;
                Ok(Self::new_directory(path, children))
            },
            FileType::Symlink => {
                let target_path = util::symlink_follow(path)?;
                Ok(Self::new_symlink(path, target_path))
            },
            other_type => {
                Err(Error::UnexpectedFileTypeError(
                    other_type,
                    path.to_path_buf(),
                ))
            },
        }
    }

    /// An iterator over `(&FsTree, PathBuf)`.
    pub fn iter(&self) -> Iter {
        Iter::new(self.files())
    }

    /// Iterator of all `FsTree`s in the structure
    pub fn files(&self) -> FilesIter {
        FilesIter::new(self)
    }

    /// Shorthand for `self.files().paths()`, see link to [`.paths()`] method
    ///
    /// [`.paths()`]: crate::iter::FilesIter::paths
    pub fn paths(&self) -> PathsIter {
        self.files().paths()
    }

    /// Fix relative paths from each node piece.
    ///
    /// If you manually build a structure like:
    ///
    /// ```plain
    /// "a": [
    ///     "b": [
    ///         "c",
    ///     ]
    /// ]
    /// ```
    ///
    /// Using the create methods, then you need to run this function to make them relative paths.
    ///
    /// ```plain
    /// "a": [
    ///     "a/b": [
    ///         "a/b/c",
    ///     ]
    /// ]
    /// ```
    ///
    /// Then, you can access any of the files only by looking at their path.
    pub fn make_paths_relative(&mut self) {
        // If this is a directory, update the path of all children
        if let TreeNode::Directory(children) = &mut self.file_type {
            for child in children.iter_mut() {
                // Update child's path
                child.path = self.path.join(&child.path);
                // Update target if it's a symlink
                if let Some(target) = child.target_mut() {
                    *target = self.path.join(&target);
                }
                child.make_paths_relative();
            }
        }
    }

    /// Makes all paths in the tree absolute.
    ///
    /// # Errors:
    ///
    /// In case `std::fs::canonicalize` fails at any path, this function will stop and return an
    /// IoError, leave the tree in a mixed state in terms of canonical paths.
    pub fn make_paths_absolute(&mut self) -> Result<()> {
        self.path = self.path.canonicalize()?;

        if let Some(children) = self.children_mut() {
            for child in children.iter_mut() {
                Self::make_paths_absolute(child)?;
            }
        }

        Ok(())
    }

    /// Merge this tree with other `FsTree`.
    ///
    /// This function is currently experimental and likely to change in future versions.
    ///
    /// # Errors:
    ///
    /// This errs if:
    ///
    /// - The trees have different roots and thus cannot be merged.
    /// - There are file conflicts.
    pub fn merge(self, other: Self) -> Option<Self> {
        if self.path != other.path {
            return None;
        }

        let path = self.path;

        match (self.file_type, other.file_type) {
            (TreeNode::Directory(left_children), TreeNode::Directory(right_children)) => {
                // TODO: Don't remake a trie here, we can use the trees directly
                // this todo needs to be solved after migrating to a proper trie
                let mut left_map: HashMap<PathBuf, FsTree> = left_children
                    .into_iter()
                    .map(|child| (child.path.clone(), child))
                    .collect();

                let mut result_vec = vec![];

                for child in right_children {
                    // If there is another one with the same path, merge them
                    match left_map.remove(&child.path) {
                        None => result_vec.push(child),
                        Some(left_equivalent) => {
                            if !child.has_same_type_as(&left_equivalent) {
                                return None;
                            } else if child.is_dir() && left_equivalent.is_dir() {
                                result_vec.push(left_equivalent.merge(child).unwrap());
                            } else {
                                result_vec.push(left_equivalent);
                                result_vec.push(child);
                            }
                        },
                    }
                }

                result_vec.extend(left_map.into_values());

                Some(Self::new_directory(path, result_vec))
            },
            _ => None,
        }
    }

    /// Reference to children vec if self.is_directory().
    pub fn children(&self) -> Option<&[Self]> {
        match &self.file_type {
            TreeNode::Directory(children) => Some(children),
            _ => None,
        }
    }

    /// Reference to children vec if self.is_directory(), mutable.
    pub fn children_mut(&mut self) -> Option<&mut Vec<Self>> {
        match &mut self.file_type {
            TreeNode::Directory(children) => Some(children),
            _ => None,
        }
    }

    /// Reference to target_path if self.is_symlink().
    pub fn target(&self) -> Option<&PathBuf> {
        match &self.file_type {
            TreeNode::Symlink(target_path) => Some(target_path),
            _ => None,
        }
    }

    /// Reference to target_path if self.is_symlink(), mutable.
    pub fn target_mut(&mut self) -> Option<&mut PathBuf> {
        match &mut self.file_type {
            TreeNode::Symlink(target_path) => Some(target_path),
            _ => None,
        }
    }

    /// Apply a closure for each direct child of this FsTree.
    ///
    /// Only 1 level deep.
    pub fn apply_to_children0(&mut self, f: impl FnMut(&mut Self)) {
        if let Some(children) = self.children_mut() {
            children.iter_mut().for_each(f);
        }
    }

    /// Apply a closure to all direct and indirect descendants inside of this structure.
    ///
    /// Calls recursively for all levels.
    pub fn apply_to_all_children1(&mut self, f: impl FnMut(&mut Self) + Copy) {
        if let Some(children) = self.children_mut() {
            children
                .iter_mut()
                .for_each(|x| x.apply_to_all_children1(f));
            children.iter_mut().for_each(f);
        }
    }

    /// Apply a closure to all direct and indirect descendants inside, also includes root.
    ///
    /// Calls recursively for all levels.
    pub fn apply_to_all(&mut self, mut f: impl FnMut(&mut Self) + Copy) {
        f(self);
        if let Some(children) = self.children_mut() {
            for child in children.iter_mut() {
                child.apply_to_all(f);
            }
        }
    }

    /// Shorthand for `file.file_type.is_regular()`
    pub fn is_regular(&self) -> bool {
        self.file_type.is_regular()
    }

    /// Shorthand for `file.file_type.is_dir()`
    pub fn is_dir(&self) -> bool {
        self.file_type.is_dir()
    }

    /// Shorthand for `file.file_type.is_symlink()`
    pub fn is_symlink(&self) -> bool {
        self.file_type.is_symlink()
    }

    /// Turn this node of the tree into a regular file.
    ///
    /// Beware the possible recursive drop of nested nodes if this node was a directory.
    pub fn to_regular(self) -> Self {
        Self {
            file_type: TreeNode::Regular,
            ..self
        }
    }

    /// Turn this node of the tree into a directory.
    ///
    /// Beware the possible recursive drop of nested nodes if this node was a directory.
    pub fn to_directory(self, children: Vec<Self>) -> Self {
        Self {
            file_type: TreeNode::Directory(children),
            ..self
        }
    }

    /// Turn this node of the tree into a symlink.
    ///
    /// Beware the possible recursive drop of nested nodes if this node was a directory.
    pub fn to_symlink(self, target_path: impl Into<PathBuf>) -> Self {
        Self {
            file_type: TreeNode::Symlink(target_path.into()),
            ..self
        }
    }

    /// Checks if the FsTree file type is the same as other FsTree.
    pub fn has_same_type_as(&self, other: &FsTree) -> bool {
        self.file_type.is_same_type_as(&other.file_type)
    }

    // /// Generate a diff from two different trees.
    // pub fn diff(&self, other: &Self) {
    //     if !self.has_same_type_as(other) {
    //         println!("Types differ! ");
    //     }

    //     let (self_children, other_children) = match (&self.file_type, &other.file_type) {
    //         (TreeNode::Directory(self_children), TreeNode::Directory(other_children)) => {
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

    /// Create the tree folder structure in the path
    pub fn create_at(&self, folder: impl AsRef<Path>) -> Result<()> {
        let folder = folder.as_ref();

        #[cfg(feature = "fs-err")]
        let symlink_function = fs_err::os::unix::fs::symlink;
        #[cfg(not(feature = "fs-err"))]
        let symlink_function = std::os::unix::fs::symlink;

        for (node, path) in self.iter() {
            let path = folder.join(&path);

            match &node.file_type {
                TreeNode::Regular => {
                    fs::File::create(path)?;
                },
                TreeNode::Directory(_) => {
                    fs::create_dir(path)?;
                },
                TreeNode::Symlink(target) => {
                    symlink_function(target, path)?;
                },
            }
        }

        Ok(())
    }

    /// Create `FsTree` in the current directory.
    ///
    /// Alias to `self.create_at(".")`.
    pub fn create(&self) -> Result<()> {
        self.create_at(".")
    }

    /// Returns a reference to the node at the path.
    ///
    /// # Examples:
    ///
    /// ```
    /// use fs_tree::FsTree;
    ///
    /// let root = FsTree::from_path_text("root/b/c/d").unwrap();
    ///
    /// // Indexing is relative from `root`, so `root` cannot be indexed.
    /// assert!(root.get("root").is_none());
    ///
    /// assert_eq!(root["b"], FsTree::from_path_text("b/c/d").unwrap());
    /// assert_eq!(root["b/c"], FsTree::from_path_text("c/d").unwrap());
    /// assert_eq!(root["b"]["c"], FsTree::from_path_text("c/d").unwrap());
    /// assert_eq!(root["b/c/d"], FsTree::from_path_text("d").unwrap());
    /// assert_eq!(root["b/c"]["d"], FsTree::from_path_text("d").unwrap());
    /// assert_eq!(root["b"]["c/d"], FsTree::from_path_text("d").unwrap());
    /// assert_eq!(root["b"]["c"]["d"], FsTree::from_path_text("d").unwrap());
    /// ```
    pub fn get(&self, path: impl AsRef<Path>) -> Option<&FsTree> {
        let path = path.as_ref();

        // Split first piece from the rest
        let (popped, path_rest) = {
            let mut iter = path.iter();
            let popped = iter.next();
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

        self.children()
            .unwrap_or(&[])
            .iter()
            .find(|child| child.path == popped)
            .and_then(|child| child.get(path_rest))
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
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;
    use crate::tree;

    // #[test]
    // #[ignore]
    // fn test_diff() {
    //     let left = FsTree::from_path_text(".config/i3/file").unwrap();
    //     let right = FsTree::from_path_text(".config/i3/folder/file/oie").unwrap();
    //     left.diff(&right);
    //     panic!();
    // }

    #[test]
    fn test_get() {
        let tree = FsTree::from_path_text("a/b/c/d").unwrap();

        // Path accesses are relative from `a`, so `a` itself cannot be indexed
        assert!(tree.get("a").is_none());

        assert_eq!(tree["b"], FsTree::from_path_text("b/c/d").unwrap());
        assert_eq!(tree["b/c"], FsTree::from_path_text("c/d").unwrap());
        assert_eq!(tree["b"]["c"], FsTree::from_path_text("c/d").unwrap());
        assert_eq!(tree["b/c/d"], FsTree::from_path_text("d").unwrap());
        assert_eq!(tree["b/c"]["d"], FsTree::from_path_text("d").unwrap());
        assert_eq!(tree["b"]["c/d"], FsTree::from_path_text("d").unwrap());
        assert_eq!(tree["b"]["c"]["d"], FsTree::from_path_text("d").unwrap());

        // Empty path returns self
        assert_eq!(tree[""], tree);
        assert_eq!(tree[""], tree[""]);
        // "."s are ignored
        assert_eq!(tree["."], tree[""]);
        assert_eq!(tree["././"], tree["."]);
        assert_eq!(tree["././."], tree);
        assert_eq!(tree["b/./."], FsTree::from_path_text("b/c/d").unwrap());
    }

    #[test]
    fn test_merge() {
        let left = FsTree::from_path_text(".config/i3/file").unwrap();
        let right = FsTree::from_path_text(".config/i3/folder/file").unwrap();
        let result = left.merge(right);

        let expected = tree! {
            ".config": {
                i3: {
                    folder: {
                        file
                    }
                    file
                }
            }
        };

        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_partial_eq_fails() {
        let left = FsTree::from_path_text(".config/i3/a").unwrap();
        let right = FsTree::from_path_text(".config/i3/b").unwrap();

        assert_ne!(left, right);
    }
}
