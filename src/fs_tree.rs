//! Implementation of [`FsTree`].

use std::{
    io,
    ops::Index,
    path::{Path, PathBuf},
};

use file_type_enum::FileType;

use crate::{
    fs,
    iter::{Iter, NodesIter, PathsIter},
    utils, Error, Result, TreeNode,
};

/// A filesystem tree recursive type.
///
/// # Iterators:
///
/// See the [iterator module documentation](crate::iter).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FsTree {
    /// The filename of this file.
    pub path: PathBuf,
    /// The TreeNode of this file.
    file_type: TreeNode,
}

impl FsTree {
    /// Construct a regular file from given value.
    pub fn new_regular(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            file_type: TreeNode::Regular,
        }
    }

    /// Construct a directory from given values.
    pub fn new_directory(path: impl Into<PathBuf>, children: Vec<Self>) -> Self {
        Self {
            path: path.into(),
            file_type: TreeNode::Directory(children),
        }
    }

    /// Construct a symlink from given values.
    pub fn new_symlink(path: impl Into<PathBuf>, target_path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            file_type: TreeNode::Symlink(target_path.into()),
        }
    }

    /// Read a `Vec<FsTree>` from the directory at `path`, follows symlinks.
    ///
    /// If you want symlink-awareness, check [`collect_from_directory_symlink`].
    ///
    /// # Errors:
    ///
    /// - If any IO error occurred.
    /// - Returns [`Error::NotADirectoryError`](crate::Error::NotADirectoryError) if the given path
    /// is not a directory.
    ///
    /// [`collect_from_directory_symlink`]: FsTree::collect_from_directory_symlink
    pub fn collect_from_directory(path: impl AsRef<Path>) -> Result<Vec<Self>> {
        Self::__collect_from_directory(path.as_ref(), true)
    }

    /// Read a `Vec<FsTree>` from the directory at `path`.
    ///
    /// If you don't want symlink-awareness, check [`collect_from_directory`].
    ///
    /// # Errors:
    ///
    /// - If any IO error occurred.
    /// - Returns [`Error::NotADirectoryError`](crate::Error::NotADirectoryError) if the given path
    /// is not a directory.
    ///
    /// [`collect_from_directory`]: FsTree::collect_from_directory
    pub fn collect_from_directory_symlink(path: impl AsRef<Path>) -> Result<Vec<Self>> {
        Self::__collect_from_directory(path.as_ref(), false)
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

    /// Construct a `FsTree` by reading from `path`, follows symlinks.
    ///
    /// If you want symlink-awareness, check [`from_path_symlink`].
    ///
    /// # Errors:
    ///
    /// - Any IO error from `fs::metadata` or `fs::read_dir`.
    /// - If any file has an unsupported file type.
    ///
    /// [`from_path_symlink`]: FsTree::from_path
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        Self::__from_path(path.as_ref(), true)
    }

    /// Construct a `FsTree` by reading from `path`.
    ///
    /// If you don't want symlink-awareness, check [`from_path`].
    ///
    /// # Errors:
    ///
    /// - Any IO error from `fs::symlink_metadata(path)` or `fs::read_dir`.
    /// - If any file has an unsupported file type.
    ///
    /// [`from_path`]: FsTree::from_path_symlink
    pub fn from_path_symlink(path: impl AsRef<Path>) -> Result<Self> {
        Self::__from_path(path.as_ref(), false)
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
                let target_path = utils::symlink_follow(path)?;
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

    /// Construct a `FsTree` from path pieces.
    ///
    /// Returns `None` if the input is empty.
    ///
    /// Returned value can correspond to a regular file or directory, but not a symlink.
    ///
    /// # Examples:
    ///
    /// ```
    /// use fs_tree::{FsTree, tree};
    ///
    /// let result = FsTree::from_path_text("a/b/c").unwrap();
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
    /// assert!(result["b"].is_dir());
    /// assert!(result["b"]["c"].is_regular());
    /// ```
    ///
    /// # Warning
    ///
    /// Inputs ending with `/`, like `Path::new("example/")` are **NOT** parsed as directories.
    ///
    /// This might change in the future, for my personal usage cases (author writing), this was
    /// always OK, but if you'd like this to change, open an issue 👍.
    pub fn from_path_text(path: impl AsRef<Path>) -> Option<Self> {
        Self::from_path_pieces(path.as_ref().iter())
    }

    /// Generic iterator version of [`from_path_text`](FsTree::from_path_text).
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

    /// Creates an iterator that yields `(&FsTree, PathBuf)`.
    ///
    /// See iterator docs at the [`iter` module documentation](crate::iter).
    pub fn iter(&self) -> Iter {
        Iter::new(self.nodes())
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
        PathsIter::new(self.iter())
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
    /// Unsolved questions: what happens if FsTree has a mismatching name?
    ///
    /// # Errors:
    ///
    /// - Returns `None` if contents of both trees conflict.
    //
    // TODO: return Result<Self, DiffNode>.
    pub fn try_merge(mut self, other: Self) -> Option<Self> {
        use TreeNode::{Directory, Regular, Symlink};

        // If types match, check if their contents match, otherwise, return `None`
        match (&mut self.file_type, other.file_type) {
            (Regular, Regular) => Some(self),
            (Symlink(left_target), Symlink(right_target)) => {
                (*left_target == right_target).then_some(self)
            },
            (Directory(left_children), Directory(right_children)) => {
                // Just to clarify it to you, we're merging the right onto the left
                let left_children: &mut Vec<FsTree> = left_children;
                let right_children: Vec<FsTree> = right_children;

                for right_child in right_children {
                    // If right_child is also found on left, try merging with left_child
                    // Else, just add it to the vec
                    if let Some(index) = left_children
                        .iter()
                        .position(|child| child.path == right_child.path)
                    {
                        let left_child = left_children.remove(index);
                        let left_child = left_child.try_merge(right_child)?;
                        left_children.push(left_child);
                    } else {
                        left_children.push(right_child);
                    }
                }

                Some(self)
            },
            // Types mismatch, not possible to merge
            (_, _) => None,
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
    pub fn write_at(&self, folder: impl AsRef<Path>) -> Result<()> {
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
    /// Alias to `self.write_at(".")`.
    pub fn create(&self) -> Result<()> {
        self.write_at(".")
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
    fn test_simple_merge() {
        let left = FsTree::from_path_text(".config/i3/file").unwrap();
        let right = FsTree::from_path_text(".config/i3/folder/file").unwrap();
        let result = left.try_merge(right);

        let expected = tree! {
            ".config": {
                i3: {
                    file
                    folder: {
                        file
                    }
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
