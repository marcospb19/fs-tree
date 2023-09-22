#![warn(missing_docs)]

//! Filesystem trie-like tree structure for commons operations.
//!
//! Given a path, you can load a `FsTree` which might represent a regular file, directory, or symlink.
//!
//! # Features:
//!
//! - Aware of regular files, directories and symlinks.
//! - Read from your filesystem.
//! - Merge trees.
//! - Get the difference of two trees.
//! - Macros for creating trees more easily (WIP).
//! - Tree iteration.
//!   - Supports useful tree filters.
//!   - You can perform operations on the iteration results (e.g. read each file and link them).
//!
//! # When not to use:
//!
//! - If you just want to iterate a directory, use [`WalkDir`] instead.
//! - If you want to use a text trie directly, use other crate too.
//!
//! # When to use:
//!
//! - You need to easily load a file type-aware trie from your filesystem and compare with other tries.
//!
//! ---
//!
//! [`WalkDir`]: https://docs.rs/walkdir

// TODO:
// - Change layout to a more trie-like tree, where the root may contain several subtrees.
//   - This helps with possible node duplication, which is bad.
//   - Also helps with complexity of queries.
// - re-test Pathsiter
// - enable all tests in rustdocs
// - improve fmt::Debug on File and TreeNode recursive display
// - add https://crates.io/crates/build-fs-tree to the list of alternatives.

/// [`FsTree`] iterators.
pub mod iter;
/// Exposed functions that are used internally by this crate.
pub mod util;

#[cfg(not(feature = "fs-err"))]
pub(crate) use std::fs;

#[cfg(feature = "fs-err")]
pub(crate) use fs_err as fs;

mod error;
mod fs_tree;
mod macros;
mod tree_node;

pub use self::{
    error::{Error, Result},
    fs_tree::FsTree,
    tree_node::TreeNode,
};
