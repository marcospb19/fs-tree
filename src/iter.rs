use crate::{File, FileType};

use std::collections::VecDeque;

pub struct FileIter<'a> {
    files_before_directories: bool,
    skip_dirs: bool,
    skip_regular_files: bool,
    skip_symlinks: bool,
    min_depth: usize,
    max_depth: usize,
    // // current_path: PathBuf,
    // // current_file_reference: &'a File,
    // Directories go at the back, files at the front
    // Has a aditional field for keeping track of depth
    file_deque: VecDeque<(&'a File, usize)>,
}

impl<'a> File {
    /// Iterator of all `File`s in the structure
    pub fn files(&'a self) -> FileIter<'a> {
        FileIter {
            files_before_directories: false,
            skip_dirs: false,
            skip_regular_files: false,
            skip_symlinks: false,
            min_depth: usize::MIN,
            max_depth: usize::MAX,
            // // current_path: self.path.clone(),
            // // current_file_reference: &self,
            file_deque: VecDeque::new(),
        }
    }

    /// Same as `.files()`, but checks if starting `File` is
    /// `FileType::Directory`
    pub fn children(&'a self) -> Option<FileIter<'a>> {
        if let FileType::Directory { .. } = self.file_type {
            Some(self.files())
        } else {
            None
        }
    }
}

impl FileIter<'_> {
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
}

impl<'a> Iterator for FileIter<'a> {
    type Item = &'a File;

    fn next(&mut self) -> Option<Self::Item> {
        let popped = if self.files_before_directories {
            self.file_deque.pop_front()
        } else {
            self.file_deque.pop_back()
        };

        // Unpack popped file and depth
        let (file, depth) = match popped {
            None => return None,
            Some(inner) => inner,
        };

        // If directory, add children, and check for `self.skip_dirs`
        if let FileType::Directory { ref children } = &file.file_type {
            for child in children {
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
        if self.skip_regular_files && file.file_type.is_file()
            || self.skip_dirs && file.file_type.is_dir()
            || self.skip_dirs && file.file_type.is_dir()
        {
            return self.next();
        }

        None
    }
}
