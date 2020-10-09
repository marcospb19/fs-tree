use crate::{error::*, file_type::FileType, FilesIter, PathsIter};

use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct File {
    pub path: PathBuf,
    pub file_type: FileType,
}

impl<'a> File {
    pub fn new(path: impl AsRef<Path>, file_type: FileType) -> Self {
        let path = path.as_ref().to_path_buf();
        File { path, file_type }
    }

    pub fn from_path(path: impl AsRef<Path>, follow_symlinks: bool) -> FSResult<Self> {
        let file_type = FileType::from_path(&path, follow_symlinks)?;
        let result = File::new(path, file_type);

        Ok(result)
    }

    /// Iterator of all `File`s in the structure
    pub fn files(&'a self) -> FilesIter<'a> {
        // Start a deque from this file, at depth 0, which can increase for each file if
        // self is a directory
        let mut file_deque = VecDeque::new();
        file_deque.push_back((self, 0));

        FilesIter {
            file_deque,
            // Default options
            ..FilesIter::default()
        }
    }

    pub fn paths(&'a self) -> PathsIter<'a> {
        self.files().paths()
    }
}
