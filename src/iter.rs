use crate::{file::File, file_type::FileType};

use std::{
    collections::VecDeque,
    ffi::OsStr,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct FilesIter<'a, T: Default> {
    // Directories go at the back, files at the front
    // Has a aditional field for keeping track of depth
    file_deque: VecDeque<(&'a File<T>, usize)>,
    // Accessed by `depth` method
    current_depth: usize,
    // Options
    files_before_directories: bool,
    skip_dirs: bool,
    skip_regular_files: bool,
    skip_symlinks: bool,
    min_depth: usize,
    max_depth: usize,
}

impl<'a, T: Default> FilesIter<'a, T> {
    // file_deque is a
    pub(crate) fn new(start_file: &'a File<T>) -> Self {
        let mut file_deque = VecDeque::new();
        // Start a deque from `start_file`, at depth 0, which can increase for each file
        // if self is a directory
        file_deque.push_back((start_file, 0));

        Self {
            file_deque,
            // Default start
            current_depth: 0,
            files_before_directories: false,
            skip_dirs: false,
            skip_regular_files: false,
            skip_symlinks: false,
            min_depth: usize::MIN,
            max_depth: usize::MAX,
        }
    }

    /// Access depth of last element, starts at 0 (root has no depth).
    pub fn depth(&self) -> usize {
        self.current_depth
    }

    pub fn paths(self) -> PathsIter<'a, T> {
        PathsIter::new(self)
    }

    // Applying filters
    pub fn files_before_directories(mut self, arg: bool) -> Self {
        self.files_before_directories = arg;
        self
    }

    pub fn skip_dirs(mut self, arg: bool) -> Self {
        self.skip_dirs = arg;
        self
    }

    pub fn skip_regular_files(mut self, arg: bool) -> Self {
        self.skip_regular_files = arg;
        self
    }

    pub fn skip_symlinks(mut self, arg: bool) -> Self {
        self.skip_symlinks = arg;
        self
    }

    pub fn min_depth(mut self, min: usize) -> Self {
        self.min_depth = min;
        self
    }

    pub fn max_depth(mut self, max: usize) -> Self {
        self.max_depth = max;
        self
    }
}

impl<'a, T: Default> Iterator for FilesIter<'a, T> {
    type Item = &'a File<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.file_deque.is_empty() {
            return None;
        }

        // Pop from left or right?
        //
        // If self.files_before_directories is set, always pop from the left, where
        // files are located
        //
        // Else, pop files from the right, that are directories, until there's no
        // directory left, then start popping from the left
        //
        // Note: last_file_is_directory <-> there is a directory in the deque
        let last_file_is_directory = self.file_deque.back().unwrap().0.file_type.is_dir();
        let pop_from_the_left = self.files_before_directories || !last_file_is_directory;

        // Unpack popped file and depth
        let (file, depth) = if pop_from_the_left {
            self.file_deque.pop_front()
        } else {
            self.file_deque.pop_back()
        }
        .unwrap();

        // Update current_depth, useful for .depth() method and PathsIter
        self.current_depth = depth;

        // If directory, add children, and check for `self.skip_dirs`
        if let FileType::Directory(ref children) = &file.file_type {
            // Reversed, because late nodes stay at the tip
            // We want at the tip the first ones
            for child in children.iter().rev() {
                if child.file_type.is_dir() {
                    self.file_deque.push_back((child, depth + 1));
                } else {
                    self.file_deque.push_front((child, depth + 1));
                }
            }
        }

        // If should skip due to depth limits
        if self.min_depth > depth || self.max_depth < depth {
            return self.next();
        }

        // If should skip due file type specific skip filters
        if self.skip_regular_files && file.file_type.is_regular()
            || self.skip_dirs && file.file_type.is_dir()
            || self.skip_dirs && file.file_type.is_dir()
        {
            return self.next();
        }

        Some(&file)
    }
}

#[derive(Debug, Clone)]
pub struct PathsIter<'a, T: Default> {
    file_iter: FilesIter<'a, T>,
    // options
    only_show_last_segment: bool,
}

impl<'a, T: Default> PathsIter<'a, T> {
    pub fn new(file_iter: FilesIter<'a, T>) -> Self {
        Self {
            file_iter,
            only_show_last_segment: false,
        }
    }

    pub fn only_show_last_segment(mut self, arg: bool) -> Self {
        self.only_show_last_segment = arg;
        self
    }

    pub fn depth(&self) -> usize {
        self.file_iter.depth()
    }

    /// True implementation of `Iterator` for `PathsIter`, without `.clone()`
    pub fn next_ref(&mut self) -> Option<&Path> {
        let file = self.file_iter.next()?;

        if self.only_show_last_segment {
            file.path.file_name().map(OsStr::as_ref)
        } else {
            Some(&file.path)
        }
    }
}

impl<T: Default> Iterator for PathsIter<'_, T> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let path_buf = self.next_ref()?.to_path_buf();
        Some(path_buf)
    }
}

#[cfg(test)]
mod tests {
    #[test] // Huge test ahead
    #[rustfmt::skip]
    fn testing_files_and_paths_iters() {
        use std::path::PathBuf;
        use crate::{File, FileType::*};

        // Implementing a syntax sugar util to make tests readable
        impl File {
            fn c(&self, index: usize) -> &File {
                &self.file_type.children().unwrap()[index]
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
        #[rustfmt::skip]
        let root = File::new(".config/", Directory(vec![
            File::new(".config/i3/", Directory(vec![
                File::new(".config/i3/file1", Regular),
                File::new(".config/i3/file2", Regular),
                File::new(".config/i3/dir/", Directory(vec![
                    File::new(".config/i3/dir/innerfile1", Regular),
                    File::new(".config/i3/dir/innerfile2", Regular)
                ])),
                File::new(".config/i3/file3", Regular),
            ])),
            File::new(".config/outerfile1", Regular),
            File::new(".config/outerfile2", Regular)
        ]));

        #[rustfmt::skip]
        // Get the references in line order, from top to bottom
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
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/
        assert_eq!(it.next(), Some(refs[5])); // .config/i3/dir/innerfile1
        assert_eq!(it.next(), Some(refs[6])); // .config/i3/dir/innerfile2
        assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
        assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
        assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
        assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
        assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2

        let mut it = root.files().files_before_directories(true);
        assert_eq!(it.next(), Some(refs[0])); // .config/
        assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
        assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
        assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
        assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/
        assert_eq!(it.next(), Some(refs[5])); // .config/i3/dir/innerfile1
        assert_eq!(it.next(), Some(refs[6])); // .config/i3/dir/innerfile2

        let mut it = root.files().skip_dirs(true);
        assert_eq!(it.next(), Some(refs[5])); // .config/i3/dir/innerfile1
        assert_eq!(it.next(), Some(refs[6])); // .config/i3/dir/innerfile2
        assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
        assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
        assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
        assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
        assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2

        let mut it = root.files().skip_regular_files(true);
        assert_eq!(it.next(), Some(refs[0])); // .config/
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/

        // min and max depth (1 <= d <= 2)
        //
        // skips:
        // .config/
        // .config/i3/dir/innerfile1
        // .config/i3/dir/innerfile2
        let mut it = root.files().min_depth(1).max_depth(2);
        assert_eq!(it.next(), Some(refs[1])); // .config/i3/
        assert_eq!(it.next(), Some(refs[4])); // .config/i3/dir/
        assert_eq!(it.next(), Some(refs[2])); // .config/i3/file1
        assert_eq!(it.next(), Some(refs[3])); // .config/i3/file2
        assert_eq!(it.next(), Some(refs[7])); // .config/i3/file3
        assert_eq!(it.next(), Some(refs[8])); // .config/outerfile1
        assert_eq!(it.next(), Some(refs[9])); // .config/outerfile2

        // Paths iterator testing
        let mut it = root.paths();
        assert_eq!(it.next().unwrap(), PathBuf::from(".config/"));                  // [0]
        assert_eq!(it.next().unwrap(), PathBuf::from(".config/i3/"));               // [1]
        assert_eq!(it.next().unwrap(), PathBuf::from(".config/i3/dir/"));           // [4]
        assert_eq!(it.next().unwrap(), PathBuf::from(".config/i3/dir/innerfile1")); // [5]
        assert_eq!(it.next().unwrap(), PathBuf::from(".config/i3/dir/innerfile2")); // [6]
        assert_eq!(it.next().unwrap(), PathBuf::from(".config/i3/file1"));          // [2]
        assert_eq!(it.next().unwrap(), PathBuf::from(".config/i3/file2"));          // [3]
        assert_eq!(it.next().unwrap(), PathBuf::from(".config/i3/file3"));          // [7]
        assert_eq!(it.next().unwrap(), PathBuf::from(".config/outerfile1"));        // [8]
        assert_eq!(it.next().unwrap(), PathBuf::from(".config/outerfile2"));        // [9]

        let mut it = root.paths().only_show_last_segment(true);
        assert_eq!(it.next().unwrap(), PathBuf::from(".config/"));   // [0]
        assert_eq!(it.next().unwrap(), PathBuf::from("i3/"));        // [1]
        assert_eq!(it.next().unwrap(), PathBuf::from("dir/"));       // [4]
        assert_eq!(it.next().unwrap(), PathBuf::from("innerfile1")); // [5]
        assert_eq!(it.next().unwrap(), PathBuf::from("innerfile2")); // [6]
        assert_eq!(it.next().unwrap(), PathBuf::from("file1"));      // [2]
        assert_eq!(it.next().unwrap(), PathBuf::from("file2"));      // [3]
        assert_eq!(it.next().unwrap(), PathBuf::from("file3"));      // [7]
        assert_eq!(it.next().unwrap(), PathBuf::from("outerfile1")); // [8]
        assert_eq!(it.next().unwrap(), PathBuf::from("outerfile2")); // [9]
    }
}
