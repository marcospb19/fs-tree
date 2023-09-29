// Dependencies of iterators:
//   `NodesIter -> Iter(NodesIter) -> PathsIter(Iter)`
//
// The most primitive iterator is `NodesIter`, `Iter` does processing to get the path, and
// `PathsIter` is basically a `std::iter::Map` to only retrieve the path.
//
// Sounds confusing, but it's actually the trivial solution that comes up when you're doing it.

//! Iterators for [`FsTree`].
//!
//! Iterators traverse in [Depth-First Order](https://en.wikipedia.org/wiki/Binary_tree#Depth-first_order).
//!
//! There are three [`FsTree`] methods for creating an iterator:
//! 1. [`.iter()`](FsTree::iter) for `Item = (&FsTree, PathBuf)`.
//! 2. [`.nodes()`](FsTree::nodes) for `Item = &FsTree`.
//! 3. [`.paths()`](FsTree::paths) for `Item = PathBuf`.
//!
//! The yielded [`PathBuf`]s correspond to the full relative path to the current node, which is the
//! result of concatenating the paths of every parent, and the current node.
//!
//! [`PathBuf`]: std::path::PathBuf
//!
//! ```
//! use fs_tree::tree;
//! use std::path::PathBuf;
//!
//! let tree = tree! {
//!     a: {
//!         b: {
//!             c
//!         }
//!     }
//! };
//!
//! let mut paths = tree.paths();
//!
//! assert_eq!(paths.next(), Some(PathBuf::from("a")));
//! assert_eq!(paths.next(), Some(PathBuf::from("a/b")));
//! assert_eq!(paths.next(), Some(PathBuf::from("a/b/c")));
//! assert_eq!(paths.next(), None);
//!
//! let mut nodes = tree.nodes();
//!
//! assert_eq!(nodes.next(), Some(&tree)); // Starts at root, it's `a` itself
//! assert_eq!(nodes.next(), Some(&tree["b"]));
//! assert_eq!(nodes.next(), Some(&tree["b/c"]));
//! assert_eq!(nodes.next(), None);
//!
//! let mut iter = tree.iter();
//!
//! iter.next();
//! assert_eq!(iter.depth(), 0);
//! iter.next();
//! assert_eq!(iter.depth(), 1);
//! iter.next();
//! assert_eq!(iter.depth(), 2);
//! ```

use std::{collections::VecDeque, path::PathBuf};

use crate::FsTree;

type NodeWithDepth<'a> = (&'a FsTree, usize);
type NodesIterDeque<'a> = VecDeque<NodeWithDepth<'a>>;

/// An iterator that runs recursively over `FsTree` structure.
#[derive(Debug, Clone)]
pub struct NodesIter<'a> {
    // Always pop from the front
    // Push to front or back, if it's a directory or not, respectively, to yield in DFS-order
    file_deque: NodesIterDeque<'a>,
    // Accessed by the `depth` method, determined by the last yielded element
    current_depth: usize,
    // Filters togglable with methods
    skip_regular_files: bool,
    skip_dirs: bool,
    skip_symlinks: bool,
    min_depth: usize,
    max_depth: usize,
}

impl<'a> NodesIter<'a> {
    /// Construct this iterator.
    pub(crate) fn new(start_file: &'a FsTree) -> Self {
        // Deque used for iterate in recursive structure
        let mut file_deque = VecDeque::new();
        // Starting deque from `start_file`, at depth 0, which can increase for each directory found
        file_deque.push_back((start_file, 0));

        Self {
            file_deque,
            current_depth: 0,
            skip_dirs: false,
            skip_regular_files: false,
            skip_symlinks: false,
            min_depth: usize::MIN,
            max_depth: usize::MAX,
        }
    }

    /// Return depth for the last yielded element.
    ///
    /// Depth `0` corresponds to the root element (first `.next()` call).
    ///
    /// # Corner cases:
    /// - If you call this function before `.next()` is called, you'll get `0`.
    /// - If `None` is yielded by this iterator, the depth value will remain immutable, and
    /// correspond to the depth of the last yielded element.
    pub fn depth(&self) -> usize {
        self.current_depth
    }

    /// Filter out regular files.
    pub fn skip_regular_files(mut self, arg: bool) -> Self {
        self.skip_regular_files = arg;
        self
    }

    /// Filter out directories.
    pub fn skip_dirs(mut self, arg: bool) -> Self {
        self.skip_dirs = arg;
        self
    }

    /// Filter out symlinks.
    pub fn skip_symlinks(mut self, arg: bool) -> Self {
        self.skip_symlinks = arg;
        self
    }

    /// Filter out entries below the given minimum [depth](Self::depth).
    pub fn min_depth(mut self, min: usize) -> Self {
        self.min_depth = min;
        self
    }

    /// Filter out entries above the given maximum [depth](Self::depth).
    pub fn max_depth(mut self, max: usize) -> Self {
        self.max_depth = max;
        self
    }
}

impl<'a> Iterator for NodesIter<'a> {
    type Item = &'a FsTree;

    fn next(&mut self) -> Option<Self::Item> {
        // Pop last element, if any
        let (file, depth) = self.file_deque.pop_front()?;

        // Update current_depth, for `.depth()` method
        self.current_depth = depth;

        // If directory, add children
        if let Some(children) = file.children() {
            // Reversed, to preserve order (push_front is different)
            for child in children.iter().rev() {
                self.file_deque.push_front((child, depth + 1));
            }
        }

        // If should skip due to any filter
        if self.skip_regular_files && file.is_regular()
            || self.skip_dirs && file.is_dir()
            || self.skip_symlinks && file.is_symlink()
            || self.min_depth > depth
            || self.max_depth < depth
        {
            // Skipping and calling the next one, if any
            return self.next();
        }

        Some(file)
    }
}

/// Tree nodes iterator.
///
/// Yields `(&FsTree, PathBuf)`.
///
/// Created by `FsTree::iter`.
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    files_iter: NodesIter<'a>,
    path_builder: PathBuf,
    previous_depth: usize,
}

impl<'a> Iter<'a> {
    /// Construct this iterator.
    pub(crate) fn new(files_iter: NodesIter<'a>) -> Self {
        Self {
            files_iter,
            path_builder: PathBuf::new(),
            previous_depth: 0,
        }
    }

    /// Return depth for the last yielded element.
    ///
    /// Depth `0` corresponds to the root element (first `.next()` call).
    ///
    /// # Corner cases:
    /// - If you call this function before `.next()` is called, you'll get `0`.
    /// - If `None` is yielded by this iterator, the depth value will remain immutable, and
    /// correspond to the depth of the last yielded element.
    pub fn depth(&self) -> usize {
        self.files_iter.depth()
    }

    /// Filter out regular files.
    pub fn skip_regular_files(mut self, arg: bool) -> Self {
        self.files_iter = self.files_iter.skip_regular_files(arg);
        self
    }

    /// Filter out directories.
    pub fn skip_dirs(mut self, arg: bool) -> Self {
        self.files_iter = self.files_iter.skip_dirs(arg);
        self
    }

    /// Filter out symlinks.
    pub fn skip_symlinks(mut self, arg: bool) -> Self {
        self.files_iter = self.files_iter.skip_symlinks(arg);
        self
    }

    /// Filter out entries below the given minimum [depth](Self::depth).
    pub fn min_depth(mut self, min: usize) -> Self {
        self.files_iter = self.files_iter.min_depth(min);
        self
    }

    /// Filter out entries above the given maximum [depth](Self::depth).
    pub fn max_depth(mut self, max: usize) -> Self {
        self.files_iter = self.files_iter.max_depth(max);
        self
    }
}

impl<'a> Iterator for Iter<'a> {
    // I'd like to return `&Path`, but the `Iterator` trait blocks putting a lifetime on `self`
    type Item = (&'a FsTree, PathBuf);

    fn next(&mut self) -> Option<Self::Item> {
        let file = self.files_iter.next()?;
        let new_depth = self.files_iter.depth();

        for _ in new_depth..=self.previous_depth {
            self.path_builder.pop();
        }
        self.path_builder.push(&file.path);

        self.previous_depth = new_depth;

        Some((file, self.path_builder.clone()))
    }
}

/// Iterator for each path inside of the recursive struct
#[derive(Debug, Clone)]
pub struct PathsIter<'a> {
    inner_iter: Iter<'a>,
}

impl<'a> PathsIter<'a> {
    /// Construct this iterator.
    pub(crate) fn new(inner_iter: Iter<'a>) -> Self {
        Self { inner_iter }
    }

    /// Return depth for the last yielded element.
    ///
    /// Depth `0` corresponds to the root element (first `.next()` call).
    ///
    /// # Corner cases:
    /// - If you call this function before `.next()` is called, you'll get `0`.
    /// - If `None` is yielded by this iterator, the depth value will remain immutable, and
    /// correspond to the depth of the last yielded element.
    pub fn depth(&self) -> usize {
        self.inner_iter.depth()
    }

    /// Filter out regular files.
    pub fn skip_regular_files(mut self, arg: bool) -> Self {
        self.inner_iter = self.inner_iter.skip_regular_files(arg);
        self
    }

    /// Filter out directories.
    pub fn skip_dirs(mut self, arg: bool) -> Self {
        self.inner_iter = self.inner_iter.skip_dirs(arg);
        self
    }

    /// Filter out symlinks.
    pub fn skip_symlinks(mut self, arg: bool) -> Self {
        self.inner_iter = self.inner_iter.skip_symlinks(arg);
        self
    }

    /// Filter out entries below the given minimum [depth](Self::depth).
    pub fn min_depth(mut self, min: usize) -> Self {
        self.inner_iter = self.inner_iter.min_depth(min);
        self
    }

    /// Filter out entries above the given maximum [depth](Self::depth).
    pub fn max_depth(mut self, max: usize) -> Self {
        self.inner_iter = self.inner_iter.max_depth(max);
        self
    }
}

impl Iterator for PathsIter<'_> {
    // I'd like to return `&Path`, but the `Iterator` trait blocks putting a lifetime on `self`
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next().map(|(_, path)| path)
    }
}

#[cfg(test)]
mod tests {
    use crate::{tree, FsTree};

    #[test]
    #[rustfmt::skip]
    fn testing_files_and_paths_iters() {
        use std::path::PathBuf;

        // Implementing a syntax sugar util to make tests readable
        impl FsTree {
            fn c(&self, index: usize) -> &FsTree {
                &self.children().unwrap()[index]
            }
        }

        // We will test the following structure:
        // ".config": [
        //     "i3": [
        //         "file1"
        //         "file2"
        //         "dir": [
        //             "innerfile1"
        //             "innerfile2"
        //         ]
        //         "file3"
        //     ]
        //     "outerfile1"
        //     "outerfile2"
        // ]

        // Create the strucutre
        let tree = tree! {
            ".config": {
                i3: {
                    file1
                    file2
                    dir: {
                        innerfile1
                        innerfile2
                    }
                    file3
                }
                outerfile1
                outerfile2
            }
        };

        // Get the references in declaration order, from top to bottom
        let refs = [
            /* 0 */ &tree,                // .config/
            /* 1 */ &tree.c(0),           // .config/i3/
            /* 2 */ &tree.c(0).c(0),      // .config/i3/file1
            /* 3 */ &tree.c(0).c(1),      // .config/i3/file2
            /* 4 */ &tree.c(0).c(2),      // .config/i3/dir/
            /* 5 */ &tree.c(0).c(2).c(0), // .config/i3/dir/innerfile1
            /* 6 */ &tree.c(0).c(2).c(1), // .config/i3/dir/innerfile2
            /* 7 */ &tree.c(0).c(3),      // .config/i3/file3
            /* 8 */ &tree.c(1),           // .config/outerfile1
            /* 9 */ &tree.c(2),           // .config/outerfile2
        ];

        let mut it = tree.nodes();
        assert_eq!(it.next(), Some(refs[0])); // .config/
        assert_eq!(it.depth(), 0);            // 0
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.depth(), 1);            // 0       1
        assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
        assert_eq!(it.depth(), 2);            // 0       1  2
        assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
        assert_eq!(it.depth(), 2);            // 0       1  2
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/
        assert_eq!(it.depth(), 2);            // 0       1  2
        assert_eq!(it.next(), Some(refs[5])); // .config/i3/dir/innerfile1
        assert_eq!(it.depth(), 3);            // 0       1  2   3
        assert_eq!(it.next(), Some(refs[6])); // .config/i3/dir/innerfile2
        assert_eq!(it.depth(), 3);            // 0       1  2   3
        assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
        assert_eq!(it.depth(), 2);            // 0       1  2
        assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
        assert_eq!(it.depth(), 1);            // 0       1
        assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2
        assert_eq!(it.depth(), 1);            // 0       1
        assert_eq!(it.next(), None);

        let mut it = tree.nodes().skip_regular_files(true);
        assert_eq!(it.next(), Some(refs[0])); // .config/
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/
        assert_eq!(it.next(), None);

        let mut it = tree.nodes().skip_dirs(true);
        assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
        assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
        assert_eq!(it.next(), Some(refs[5])); // .config/i3/dir/innerfile1
        assert_eq!(it.next(), Some(refs[6])); // .config/i3/dir/innerfile2
        assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
        assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
        assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2
        assert_eq!(it.next(), None);

        let mut it = tree.nodes().skip_regular_files(true);
        assert_eq!(it.next(), Some(refs[0])); // .config/
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/

        // min and max depth (1 <= d <= 2)
        //
        // skips:
        // .config/
        // .config/i3/dir/innerfile1
        // .config/i3/dir/innerfile2
        let mut it = tree.nodes().min_depth(1).max_depth(2);
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
        assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/
        assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
        assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
        assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2
        assert_eq!(it.next(), None);

        // Paths iterator testing
        let p = PathBuf::from;
        let mut it = tree.paths();
        assert_eq!(it.next(), Some(p(".config/")));                  // [0]
        assert_eq!(it.next(), Some(p(".config/i3/")));               // [1]
        assert_eq!(it.next(), Some(p(".config/i3/file1")));          // [2]
        assert_eq!(it.next(), Some(p(".config/i3/file2")));          // [3]
        assert_eq!(it.next(), Some(p(".config/i3/dir/")));           // [4]
        assert_eq!(it.next(), Some(p(".config/i3/dir/innerfile1"))); // [5]
        assert_eq!(it.next(), Some(p(".config/i3/dir/innerfile2"))); // [6]
        assert_eq!(it.next(), Some(p(".config/i3/file3")));          // [7]
        assert_eq!(it.next(), Some(p(".config/outerfile1")));        // [8]
        assert_eq!(it.next(), Some(p(".config/outerfile2")));        // [9]
        assert_eq!(it.next(), None);
    }
}
