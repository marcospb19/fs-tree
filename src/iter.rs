//! Iterators for [`FsTree`].
//!
//! Iterators traverse in [Depth-First Order](https://en.wikipedia.org/wiki/Binary_tree#Depth-first_order).
//!
//! There are three [`FsTree`] methods for creating an iterator:
//! 1. [`Iter`](iter::Iter) from [`.iter()`](FsTree::iter) yields `(&FsTree, PathBuf)`.
//! 2. [`NodesIter`](iter::NodesIter) from [`.nodes()`](FsTree::nodes) yields `&FsTree`.
//! 3. [`PathsIter`](iter::PathsIter) from [`.paths()`](FsTree::paths) yields `PathBuf`.
//!
//! The yielded [`PathBuf`]s correspond to the full relative path to the current node, which is the
//! result of concatenating the paths of every parent, and the current node.
//!
//! [`PathBuf`]: std::path::PathBuf
//!
//! # Examples:
//!
//! ```
//! use fs_tree::tree;
//! use std::path::PathBuf;
//!
//! let tree = tree! {
//!     dir: [
//!         file1
//!         file2
//!         file3
//!     ]
//! };
//!
//!
//! let mut paths = tree.paths();
//! assert_eq!(paths.next(), Some(PathBuf::from(""))); // Root can be skipped with `.min_depth(1)`
//! assert_eq!(paths.next(), Some(PathBuf::from("dir")));
//! assert_eq!(paths.next(), Some(PathBuf::from("dir/file1")));
//! assert_eq!(paths.next(), Some(PathBuf::from("dir/file2")));
//! assert_eq!(paths.next(), Some(PathBuf::from("dir/file3")));
//! assert_eq!(paths.next(), None);
//!
//! let mut nodes = tree.nodes();
//! assert_eq!(nodes.next(), Some(&tree));
//! assert_eq!(nodes.next(), Some(&tree["dir"]));
//! assert_eq!(nodes.next(), Some(&tree["dir/file1"]));
//! assert_eq!(nodes.next(), Some(&tree["dir/file2"]));
//! assert_eq!(nodes.next(), Some(&tree["dir/file3"]));
//! assert_eq!(nodes.next(), None);
//! ```

use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use crate::FsTree;

type NodeWithPathAndDepth<'a> = (&'a FsTree, usize, &'a Path);
type NodesIterDeque<'a> = VecDeque<NodeWithPathAndDepth<'a>>;

/// This is the underlying iterator implementation for the other iterators.
///
/// It does not implement the `Iterator` trait, instead, it has its own `.next()` method, because
/// GATs is not stabilized and returning `&Path` makes this a lending iterator.
#[derive(Debug, Clone)]
struct InnerIter<'a> {
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

    /// TODO: what is this
    last_path: &'a Path,
}

impl<'a> InnerIter<'a> {
    fn new(start_file: &'a FsTree) -> Self {
        // Deque used for iterate in recursive structure
        let mut file_deque = VecDeque::new();
        // Starting deque from `start_file`, at depth 0, which can increase for each directory found
        file_deque.push_back((start_file, 0, Path::new("")));

        Self {
            file_deque,
            current_depth: 0,
            skip_dirs: false,
            skip_regular_files: false,
            skip_symlinks: false,
            min_depth: usize::MIN,
            max_depth: usize::MAX,
            last_path: Path::new(""),
        }
    }

    /// Let other iterators access the inner Path reference.
    fn last_path(&self) -> &Path {
        self.last_path
    }

    fn depth(&self) -> usize {
        self.current_depth
    }
}

impl<'a> Iterator for InnerIter<'a> {
    type Item = &'a FsTree;

    fn next(&mut self) -> Option<Self::Item> {
        // Pop last element, if any
        let (file, depth, last_path) = self.file_deque.pop_front()?;

        // Update current_depth, for `.depth()` method
        self.current_depth = depth;

        // If directory, add children
        if let Some(children) = file.children() {
            // Reversed, to preserve order (push_front is different)
            for (path, child) in children.iter().rev() {
                self.file_deque.push_front((child, depth + 1, path));
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

        self.last_path = last_path;

        Some(file)
    }
}

macro_rules! impl_iter_methods {
    ($($path_to_the_inner_iter:tt)*) => {
        /// Return depth for the last yielded element.
        ///
        /// Depth `0` corresponds to the root element (first `.next()` call).
        ///
        /// # Corner cases:
        /// - If you call this function before `.next()` is called, you'll get `0`.
        /// - If `None` is yielded by this iterator, the depth value will remain immutable, and
        ///   correspond to the depth of the last yielded element.
        pub fn depth(&self) -> usize {
            self.$($path_to_the_inner_iter)*.depth()
        }

        /// Filter out regular files.
        pub fn skip_regular_files(mut self, arg: bool) -> Self {
            self.$($path_to_the_inner_iter)*.skip_regular_files = arg;
            self
        }

        /// Filter out directories.
        pub fn skip_dirs(mut self, arg: bool) -> Self {
            self.$($path_to_the_inner_iter)*.skip_dirs = arg;
            self
        }

        /// Filter out symlinks.
        pub fn skip_symlinks(mut self, arg: bool) -> Self {
            self.$($path_to_the_inner_iter)*.skip_symlinks = arg;
            self
        }

        /// Filter out entries below the given minimum [depth](Self::depth).
        pub fn min_depth(mut self, min: usize) -> Self {
            self.$($path_to_the_inner_iter)*.min_depth = min;
            self
        }

        /// Filter out entries above the given maximum [depth](Self::depth).
        pub fn max_depth(mut self, max: usize) -> Self {
            self.$($path_to_the_inner_iter)*.max_depth = max;
            self
        }
    };
}

/// Tree nodes iterator.
///
/// Yields `(&FsTree, PathBuf)`.
///
/// Created by `FsTree::iter`.
#[derive(Debug, Clone)]
pub struct NodesIter<'a> {
    inner_iter: InnerIter<'a>,
}

impl<'a> NodesIter<'a> {
    pub(crate) fn new(root: &'a FsTree) -> Self {
        Self {
            inner_iter: InnerIter::new(root),
        }
    }

    impl_iter_methods!(inner_iter);
}

impl<'a> Iterator for NodesIter<'a> {
    type Item = &'a FsTree;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

/// Tree iterator.
///
/// Yields `(&FsTree, PathBuf)`.
///
/// Created by `FsTree::iter`.
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    inner_iter: InnerIter<'a>,
    path_builder: PathBuf,
    previous_depth: usize,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(root: &'a FsTree) -> Self {
        Self {
            inner_iter: InnerIter::new(root),
            path_builder: PathBuf::new(),
            previous_depth: 0,
        }
    }

    impl_iter_methods!(inner_iter);
}

impl<'a> Iterator for Iter<'a> {
    // I'd like to return `&Path`, but the `Iterator` trait blocks putting a lifetime on `self`
    type Item = (&'a FsTree, PathBuf);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.inner_iter.next()?;
        let new_depth = self.inner_iter.depth();
        let last_path = self.inner_iter.last_path();

        for _ in new_depth..=self.previous_depth {
            self.path_builder.pop();
        }

        self.path_builder.push(last_path);

        self.previous_depth = new_depth;

        Some((node, self.path_builder.clone()))
    }
}

/// Iterator for each path inside of the recursive struct
#[derive(Debug, Clone)]
pub struct PathsIter<'a> {
    iter: Iter<'a>,
}

impl<'a> PathsIter<'a> {
    pub(crate) fn new(root: &'a crate::FsTree) -> Self {
        Self {
            iter: Iter::new(root),
        }
    }

    impl_iter_methods!(iter.inner_iter);
}

impl Iterator for PathsIter<'_> {
    // Can't return `&Path` because we don't have GATs yet
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, path)| path)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::tree;

    #[test]
    #[rustfmt::skip]
    fn testing_files_and_paths_iters() {
        // Create the strucutre
        let tree = tree! {
            ".config": [
                i3: [
                    file1
                    file2
                    dir: [
                        innerfile1
                        innerfile2
                    ]
                    file3
                ]
                outerfile1
                outerfile2
            ]
        };

        // Get the references in ascending order
        let refs = [
            /* 0 */ &tree,
            /* 1 */ &tree[".config/"],
            /* 2 */ &tree[".config/i3/"],
            /* 5 */ &tree[".config/i3/dir/"],
            /* 6 */ &tree[".config/i3/dir/innerfile1"],
            /* 7 */ &tree[".config/i3/dir/innerfile2"],
            /* 3 */ &tree[".config/i3/file1"],
            /* 4 */ &tree[".config/i3/file2"],
            /* 8 */ &tree[".config/i3/file3"],
            /* 9 */ &tree[".config/outerfile1"],
            /* 0 */ &tree[".config/outerfile2"],
        ];

        // Paths iterator testing
        let mut it = tree.paths();
        assert_eq!(it.next(), Some("".into()));
        assert_eq!(it.next(), Some(".config/".into()));
        assert_eq!(it.next(), Some(".config/i3/".into()));
        assert_eq!(it.next(), Some(".config/i3/dir/".into()));
        assert_eq!(it.next(), Some(".config/i3/dir/innerfile1".into()));
        assert_eq!(it.next(), Some(".config/i3/dir/innerfile2".into()));
        assert_eq!(it.next(), Some(".config/i3/file1".into()));
        assert_eq!(it.next(), Some(".config/i3/file2".into()));
        assert_eq!(it.next(), Some(".config/i3/file3".into()));
        assert_eq!(it.next(), Some(".config/outerfile1".into()));
        assert_eq!(it.next(), Some(".config/outerfile2".into()) );
        assert_eq!(it.next(), None);

        // This
        let mut it = tree.nodes();
        assert_eq!(it.next(), Some(refs[0]));  // ""
        assert_eq!(it.depth(), 0);             //
        assert_eq!(it.next(), Some(refs[1]));  // ".config/"
        assert_eq!(it.depth(), 1);             //  1
        assert_eq!(it.next(), Some(refs[2]));  // ".config/i3/"
        assert_eq!(it.depth(), 2);             //  1       2
        assert_eq!(it.next(), Some(refs[3]));  // ".config/i3/dir/"
        assert_eq!(it.depth(), 3);             //  1       2
        assert_eq!(it.next(), Some(refs[4]));  // ".config/i3/dir/innerfile1"
        assert_eq!(it.depth(), 4);             //  1       2  3   4
        assert_eq!(it.next(), Some(refs[5]));  // ".config/i3/dir/innerfile2"
        assert_eq!(it.depth(), 4);             //  1       2  3   4
        assert_eq!(it.next(), Some(refs[6]));  // ".config/i3/file1"
        assert_eq!(it.depth(), 3);             //  1       2  3
        assert_eq!(it.next(), Some(refs[7]));  // ".config/i3/file2"
        assert_eq!(it.depth(), 3);             //  1       2  3
        assert_eq!(it.next(), Some(refs[8]));  // ".config/i3/file3"
        assert_eq!(it.depth(), 3);             //  1       2  3
        assert_eq!(it.next(), Some(refs[9]));  // ".config/outerfile1"
        assert_eq!(it.depth(), 2);             //  1       2
        assert_eq!(it.next(), Some(refs[10])); // ".config/outerfile2"
        assert_eq!(it.depth(), 2);             //  1       2
        assert_eq!(it.next(), None);

        let mut it = tree.nodes().skip_regular_files(true);
        assert_eq!(it.next(), Some(refs[0]));  // ""
        assert_eq!(it.next(), Some(refs[1]));  // ".config/"
        assert_eq!(it.next(), Some(refs[2]));  // ".config/i3/"
        assert_eq!(it.next(), Some(refs[3]));  // ".config/i3/dir/"
        assert_eq!(it.next(), None);

        let mut it = tree.nodes().skip_dirs(true);
        assert_eq!(it.next(), Some(refs[4]));  // ".config/i3/dir/innerfile1"
        assert_eq!(it.next(), Some(refs[5]));  // ".config/i3/dir/innerfile2"
        assert_eq!(it.next(), Some(refs[6]));  // ".config/i3/file1"
        assert_eq!(it.next(), Some(refs[7]));  // ".config/i3/file2"
        assert_eq!(it.next(), Some(refs[8]));  // ".config/i3/file3"
        assert_eq!(it.next(), Some(refs[9]));  // ".config/outerfile1"
        assert_eq!(it.next(), Some(refs[10])); // ".config/outerfile2"
        assert_eq!(it.next(), None);

        // min and max depth (2 <= d <= 2  =>  d == 2)
        //
        // skips:
        // ""
        // ".config/"
        // ".config/i3/dir/innerfile1"
        // ".config/i3/dir/innerfile2"
        let mut it = tree.nodes().min_depth(2).max_depth(2);
        assert_eq!(it.next(), Some(refs[2]));  // ".config/i3/"
        assert_eq!(it.next(), Some(refs[9]));  // ".config/outerfile1"
        assert_eq!(it.next(), Some(refs[10])); // ".config/outerfile2"
        assert_eq!(it.next(), None);
    }
}
