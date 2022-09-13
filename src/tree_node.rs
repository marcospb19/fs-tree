use std::{mem, path::PathBuf};

use crate::FsTree;

/// A filesystem tree recursive enum.
///
/// This enum has a variant for the following file types:
/// 1. `TreeNode::Regular` - A regular file.
/// 2. `TreeNode::Directory` - A folder with a (possible empty) list of children.
/// 3. `TreeNode::Symlink` - A symbolic link that points to another path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TreeNode {
    /// A regular file.
    Regular,
    /// A directory, might have children `FsTree`s inside.
    Directory(Vec<FsTree>),
    /// Symbolic link, and it's target path.
    ///
    /// The link might be broken, it's not guaranteed that a symlink points to a valid path.
    Symlink(PathBuf),
}

impl TreeNode {
    /// Checks if the TreeNode is the same type as other.
    pub fn is_same_type_as(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }

    /// Shorthand for `file.file_type.is_regular()`
    pub fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }

    /// Shorthand for `file.file_type.is_dir()`
    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Directory(_))
    }

    /// Shorthand for `file.file_type.is_symlink()`
    pub fn is_symlink(&self) -> bool {
        matches!(self, Self::Symlink(_))
    }

    /// Displays the file type discriminant str.
    pub fn file_type_display(&self) -> &'static str {
        match self {
            Self::Regular => "regular file",
            Self::Directory(_) => "directory",
            Self::Symlink(_) => "symlink",
        }
    }
}

#[cfg(feature = "libc-file-type")]
impl TreeNode {
    /// Returns the file type equivalent [`libc::mode_t`] value.
    pub fn as_mode_t(&self) -> libc::mode_t {
        match self {
            TreeNode::Regular => libc::S_IFREG,
            TreeNode::Directory(_) => libc::S_IFDIR,
            TreeNode::Symlink(_) => libc::S_IFCHR,
        }
    }
}
