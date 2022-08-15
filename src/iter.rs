use std::{
    collections::VecDeque as Deque,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::FileTree;

/// An iterator that runs recursively over `FileTree` structure.
#[derive(Debug, Clone)]
pub struct FilesIter<'a> {
    // Pop from the front, push to front or back, it depends
    // Cause when we open a directory, we need to traverse it's content first
    file_deque: Deque<(&'a FileTree, usize)>,
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
    pub(crate) fn new(start_file: &'a FileTree) -> Self {
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

    /// Retrieve files before the directories when possible
    ///
    /// If true, when reading files inside a directory, put it in the start or
    /// end of the Deque in a way that is deterministic
    ///
    /// Example:
    /// ```txt
    /// //// This was commented out because we don't have nice macros to make this example convenient
    /// // let mut it = root.files().files_before_directories(true);
    /// // assert_eq!(it.next(), Some(refs[0])); // .config/
    /// // assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
    /// // assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2
    /// // assert_eq!(it.next(), Some(refs[1])); // .config/i3/
    /// // assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
    /// // assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
    /// // assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
    /// // assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/
    /// // assert_eq!(it.next(), Some(refs[5])); // .config/i3/dir/innerfile1
    /// // assert_eq!(it.next(), Some(refs[6])); // .config/i3/dir/innerfile2
    /// ```
    ///
    /// Se above that when we enter a directory, first we go into the files in
    /// that directory, and then in the files of that directory, this guarantees
    /// that when we see depth of `X`, we can only see depth of `X` or `X + 1`
    /// for the next call.

    // Removed option, I don't think it'll come back
    // pub fn files_before_directories(mut self, arg: bool) -> Self {
    //     self.files_before_directories = arg;
    //     self
    // }

    /// Filter out every `FileTree::Regular`
    pub fn skip_regular_files(mut self, arg: bool) -> Self {
        self.skip_regular_files = arg;
        self
    }

    /// Filter out every `FileTree::Directory`
    pub fn skip_dirs(mut self, arg: bool) -> Self {
        self.skip_dirs = arg;
        self
    }

    /// Filter out every `FileTree::Symlink`
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
    type Item = &'a FileTree;

    fn next(&mut self) -> Option<Self::Item> {
        // Pop last element, if any
        let (file, depth) = self.file_deque.pop_front()?;

        // Update current_depth, for `.depth()` method
        self.current_depth = depth;

        // a b c
        // b c
        // b1 b2 c
        // b2 c
        // c
        // _

        /*
            a,
            b [ b1, b2],
            c,
        */

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

/// Iterator for each path inside of the recursive struct
#[derive(Debug, Clone)]
pub struct PathsIter<'a> {
    file_iter: FilesIter<'a>,
    // options
    only_show_last_segment: bool,
}

impl<'a> PathsIter<'a> {
    // Used by `FilesIter::paths(self)`
    fn new(file_iter: FilesIter<'a>) -> Self {
        Self {
            file_iter,
            only_show_last_segment: false,
        }
    }

    /// Apply `Path::file_name` to each iteration
    pub fn only_show_last_segment(mut self, arg: bool) -> Self {
        self.only_show_last_segment = arg;
        self
    }

    /// Query for depth of the last element
    ///
    /// Same as FilesIter.depth()
    pub fn depth(&self) -> usize {
        self.file_iter.depth()
    }

    /// Underlying implementation of `Iterator` for `PathsIter`, without any
    /// `.clone()`
    pub fn next_ref(&mut self) -> Option<&Path> {
        let file = self.file_iter.next()?;

        if self.only_show_last_segment {
            file.path().file_name().map(OsStr::as_ref)
        } else {
            Some(file.path())
        }
    }
}

impl Iterator for PathsIter<'_> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let path_buf = self.next_ref()?.to_path_buf();
        Some(path_buf)
    }
}

#[cfg(test)]
mod tests {
    use crate::FileTree;

    #[test]
    #[rustfmt::skip]
    fn testing_files_and_paths_iters() {
        use std::path::PathBuf;

        // Implementing a syntax sugar util to make tests readable
        impl FileTree {
            fn c(&self, index: usize) -> &FileTree {
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

        // This should be replaced with a proper macro
        //
        // Create the strucutre
        let root = {
            // dir!(".config", [
            //      dir!("i3", )
            // ])
            FileTree::new_directory(".config/", vec![
                FileTree::new_directory(".config/i3/", vec![
                    FileTree::new_regular(".config/i3/file1"),
                    FileTree::new_regular(".config/i3/file2"),
                    FileTree::new_directory(".config/i3/dir/", vec![
                        FileTree::new_regular(".config/i3/dir/innerfile1"),
                        FileTree::new_regular(".config/i3/dir/innerfile2")
                    ]),
                    FileTree::new_regular(".config/i3/file3"),
                ]),
                FileTree::new_regular(".config/outerfile1"),
                FileTree::new_regular(".config/outerfile2")
            ])
        };

        // Get the references in declaration order, from top to bottom
        let refs = vec![
            /* 0 */ &root,                // .config/
            /* 1 */ &root.c(0),           // .config/i3/
            /* 2 */ &root.c(0).c(0),      // .config/i3/file1
            /* 3 */ &root.c(0).c(1),      // .config/i3/file2
            /* 4 */ &root.c(0).c(2),      // .config/i3/dir/
            /* 5 */ &root.c(0).c(2).c(0), // .config/i3/dir/innerfile1
            /* 6 */ &root.c(0).c(2).c(1), // .config/i3/dir/innerfile2
            /* 7 */ &root.c(0).c(3),      // .config/i3/file3
            /* 8 */ &root.c(1),           // .config/outerfile1
            /* 9 */ &root.c(2),           // .config/outerfile2
        ];

        let mut it = root.files();
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

        let mut it = root.files().skip_regular_files(true);
        assert_eq!(it.next(), Some(refs[0])); // .config/
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/
        assert_eq!(it.next(), None);

        let mut it = root.files().skip_dirs(true);
        assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
        assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
        assert_eq!(it.next(), Some(refs[5])); // .config/i3/dir/innerfile1
        assert_eq!(it.next(), Some(refs[6])); // .config/i3/dir/innerfile2
        assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
        assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
        assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2
        assert_eq!(it.next(), None);

        let mut it = root.files().skip_regular_files(true);
        assert_eq!(it.next(), Some(refs[0])); // .config/
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/

        // let mut it = root.files();
        // assert_eq!(it.next(), Some(refs[0])); // .config/
        // assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        // assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
        // assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
        // assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/
        // assert_eq!(it.next(), Some(refs[5])); // .config/i3/dir/innerfile1
        // assert_eq!(it.next(), Some(refs[6])); // .config/i3/dir/innerfile2
        // assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
        // assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
        // assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2
        // assert_eq!(it.next(), None);




        // min and max depth (1 <= d <= 2)
        //
        // skips:
        // .config/
        // .config/i3/dir/innerfile1
        // .config/i3/dir/innerfile2
        let mut it = root.files().min_depth(1).max_depth(2);
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
        assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/
        assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
        assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
        assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2
        assert_eq!(it.next(), None);

        // assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        // assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/
        // assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
        // assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
        // assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
        // assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
        // assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2

        // Paths iterator testing
        let p = PathBuf::from;
        let mut it = root.paths();
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

        let mut it = root.paths().only_show_last_segment(true);
        assert_eq!(it.next(), Some(p(".config/")));                  // [0]
        assert_eq!(it.next(), Some(p("i3/")));               // [1]
        assert_eq!(it.next(), Some(p("file1")));          // [2]
        assert_eq!(it.next(), Some(p("file2")));          // [3]
        assert_eq!(it.next(), Some(p("dir/")));           // [4]
        assert_eq!(it.next(), Some(p("innerfile1"))); // [5]
        assert_eq!(it.next(), Some(p("innerfile2"))); // [6]
        assert_eq!(it.next(), Some(p("file3")));          // [7]
        assert_eq!(it.next(), Some(p("outerfile1")));        // [8]
        assert_eq!(it.next(), Some(p("outerfile2")));        // [9]
        assert_eq!(it.next(), None);
    }
}
