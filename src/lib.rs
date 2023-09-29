//! [`FsTree`] is a path [Trie] with an API focused on filesystem operations.
//!
//! [Trie]: https://en.wikipedia.org/wiki/Trie
//!
//! # Clarifications:
//!
//! 1. Unix only.
//! 2. This crate was transfered after `0.1.3` and changed purpose.
//! 3. This crate refers to _"filesystem tree"_ as the result you get from recursively traversing files:
//!     - If you try traversing a single file, you get a single node.
//!     - If you try traversing a directories, you might get a large subtree (of nodes).
//!     - This is agnostic to the filesystem underlying system (nothing to do with `ext4` or `btrfs`).
//!
//! A tree is made out of three file types:
//!
//! ```
//! # use {fs_tree::FsTree, std::path::PathBuf};
//! pub enum TreeNode {
//!     Regular,                // Leaf node
//!     Directory(Vec<FsTree>), // Recursive part, is a leaf only if Vec is empty
//!     Symlink(PathBuf),       // Leaf node
//! }
//! ```
//!
//! A `FsTree` is a node with it's path piece.
//!
//! ```
//! # use {fs_tree::TreeNode, std::path::PathBuf};
//! pub struct FsTree {
//!     /// The filename of this file.
//!     pub path: PathBuf,
//!     /// The TreeNode of this file.
//!     pub file_type: TreeNode,
//! }
//! ```
//!
//! That makes it a [Trie].
//!
//! Like `std` functions, functions in this crate follow symlinks (and symlink chains), so you'll
//! never get a `TreeNode::Symlink(_)` in your tree! However, if you want symlink-awareness, use the
//! function version with the `symlink` suffix (see [`FsTree::from_path`] vs
//! [`FsTree::from_path_symlink`]).
//!
//! Ways to construct a [`FsTree`]:
//! 1. Load node from path.
//! 2. Load a `Vec` of nodes from a folder.
//! 3. Declare a `FsTree` literal with the macros [`tree!`] and [`trees!`].
//! 4. Parse from path text segments. ([`FsTree::from_path_text`])
//!
//! What you can do with a [`FsTree`]:
//! 1. Merge with another tree. ([`FsTree::try_merge`])
//! 2. Traverse it. ([`iter` module](crate::iter))
//! 3. Write it to disk. ([`FsTree::write_at`])
//! 4. Compare with another `FsTree`, generating a DiffTree. (TODO)
//! 5. Modify it at a specific path.
//! 6. Apply a closure on all nodes.
//! 7. Assert it (compare with the macro literal).
//!
//! # Alternatives:
//! - Crate [`walkdir`](https://crates.io/crates/walkdir) - Better if you just need to iterate on
//! filesystem trees.
//! - Crate [`build-fs-tree`](https://crates.io/crates/build-fs-tree) - If you need to create a
//! filesystem tree from a YAML file.
//!     - The closest we got is creating a tree with [`tree!`](crate::tree), and persisting it on
//! the disk with [`write_at`](FsTree::write_at).

#![warn(missing_docs)]
// Emits false-positives on macros.
#![allow(clippy::vec_init_then_push)]

/// [`FsTree`] iterators.
pub mod iter;

pub use self::{
    error::{Error, Result},
    fs_tree::FsTree,
    tree_node::TreeNode,
};

mod error;
mod fs_tree;
mod macros;
mod tree_node;
pub(crate) mod utils;

#[cfg(not(feature = "fs-err"))]
pub(crate) use std::fs;

#[cfg(feature = "fs-err")]
pub(crate) use fs_err as fs;
