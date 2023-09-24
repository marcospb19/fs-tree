use std::{collections::VecDeque as Deque, path::PathBuf};

use crate::FsTree;

/// An iterator that runs recursively over `FsTree` structure.
#[derive(Debug, Clone)]
pub struct FilesIter<'a> {
    // Pop from the front, push to front or back, it depends
    // Cause when we open a directory, we need to traverse it's content first
    file_deque: Deque<(&'a FsTree, usize)>,
    // Accessed by `depth` method, determined by the last yielded element
    current_depth: usize,

    // Filters public via methods
    skip_regular_files: bool,
    skip_dirs: bool,
    skip_symlinks: bool,
    min_depth: usize,
    max_depth: usize,
}

impl<'a> FilesIter<'a> {
    pub(crate) fn new(start_file: &'a FsTree) -> Self {
        // Deque used for iterate in recursive structure
        let mut file_deque = Deque::new();
        // Starting deque from `start_file`, at depth 0, which can increase for each file
        // if self is a directory
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

    /// Return depth in the tree of the last element yielded.
    ///
    /// If called AFTER first `.next()` call, returns 0 (root has no depth).
    ///
    /// You shouldn't do these, but if you do:
    /// - If you call BEFORE any `.next()`, will be 0.
    /// - If you call AFTER None is yielded, will return the last Some(val) depth.
    pub fn depth(&self) -> usize {
        self.current_depth
    }

    /// Consume iterator, turns into `PathsIter`
    pub fn paths(self) -> PathsIter<'a> {
        PathsIter::new(self)
    }

    /// Filter out every `FsTree::Regular`
    pub fn skip_regular_files(mut self, arg: bool) -> Self {
        self.skip_regular_files = arg;
        self
    }

    /// Filter out every `FsTree::Directory`
    pub fn skip_dirs(mut self, arg: bool) -> Self {
        self.skip_dirs = arg;
        self
    }

    /// Filter out every `FsTree::Symlink`
    pub fn skip_symlinks(mut self, arg: bool) -> Self {
        self.skip_symlinks = arg;
        self
    }

    /// Filter out all the next entries that are below a minimum depth
    pub fn min_depth(mut self, min: usize) -> Self {
        self.min_depth = min;
        self
    }

    /// Filter out all the next entries that are above a maximum depth
    pub fn max_depth(mut self, max: usize) -> Self {
        self.max_depth = max;
        self
    }
}

impl<'a> Iterator for FilesIter<'a> {
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
    files_iter: FilesIter<'a>,
    path_builder: PathBuf,
    previous_depth: usize,
}

impl<'a> Iter<'a> {
    // Used by `FilesIter::paths(self)`
    pub(crate) fn new(files_iter: FilesIter<'a>) -> Self {
        Self {
            files_iter,
            path_builder: PathBuf::new(),
            previous_depth: 0,
        }
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
    files_iter: FilesIter<'a>,
    path_builder: PathBuf,
    previous_depth: usize,
}

impl<'a> PathsIter<'a> {
    fn new(files_iter: FilesIter<'a>) -> Self {
        Self {
            files_iter,
            path_builder: PathBuf::new(),
            previous_depth: 0,
        }
    }
}

impl Iterator for PathsIter<'_> {
    // I'd like to return `&Path`, but the `Iterator` trait doesn't allow a lifetime parameter on `self`
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let file = self.files_iter.next()?;

        let new_depth = self.files_iter.depth();
        for _ in new_depth..=self.previous_depth {
            self.path_builder.pop();
        }
        self.path_builder.push(&file.path);

        self.previous_depth = new_depth;

        Some(self.path_builder.clone())
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

        let mut it = tree.files();
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

        let mut it = tree.files().skip_regular_files(true);
        assert_eq!(it.next(), Some(refs[0])); // .config/
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/
        assert_eq!(it.next(), None);

        let mut it = tree.files().skip_dirs(true);
        assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
        assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
        assert_eq!(it.next(), Some(refs[5])); // .config/i3/dir/innerfile1
        assert_eq!(it.next(), Some(refs[6])); // .config/i3/dir/innerfile2
        assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
        assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
        assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2
        assert_eq!(it.next(), None);

        let mut it = tree.files().skip_regular_files(true);
        assert_eq!(it.next(), Some(refs[0])); // .config/
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/

        // min and max depth (1 <= d <= 2)
        //
        // skips:
        // .config/
        // .config/i3/dir/innerfile1
        // .config/i3/dir/innerfile2
        let mut it = tree.files().min_depth(1).max_depth(2);
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
