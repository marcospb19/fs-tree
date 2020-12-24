use crate::{
    error::*,
    file_type::FileType,
    iter::{FilesIter, PathsIter},
};

use std::{
    fmt,
    path::{Path, PathBuf},
};

/// Recursive file representation that supports a generic extra field
///
/// For now, all paths are made of multiple components, for example:
///
/// ```txt
/// "a": [
///     "b",
///     "c",
/// ]
/// ```
///
/// The inner files path is "a/b" and "a/c" instead of just "b" or "c"
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct File<T> {
    /// Relative path to File
    pub path: PathBuf,
    /// The recursive type of the file
    pub file_type: FileType<T>,
    /// Optional customizable field
    pub extra: Option<T>,
}

impl<T> File<T> {
    /// Create `File` from arguments
    ///
    /// This function will panic if you pass a path with multiple components to
    /// it, because it breaks iterators functionality.
    pub fn new(path: impl AsRef<Path>, file_type: FileType<T>) -> Self {
        // Todo: remove this and update docs!
        if !file_type.is_dir() && path.as_ref().components().count() > 1 {
            // panic!("Not a directory and has more than one component");
            // return Err(FsError::NotADirectory);
        }

        unsafe { File::new_unchecked(path, file_type) }
    }

    /// Create `File` from arguments
    ///
    /// Should be unsafe?
    ///
    /// # Safety
    /// The behavior might be undefined if the `path` has more than one
    /// `component`
    pub unsafe fn new_unchecked(path: impl AsRef<Path>, file_type: FileType<T>) -> Self {
        File {
            path: path.as_ref().to_path_buf(),
            file_type,
            extra: None,
        }
    }

    /// Create `File` reading from the `path`
    ///
    /// Accesses the filesystem to build it up
    pub fn from_path(path: impl AsRef<Path>, follow_symlinks: bool) -> FsResult<Self> {
        let file_type = FileType::from_path(&path, follow_symlinks)?;
        let result = File::new(path, file_type);

        Ok(result)
    }

    /// Create `File` structure from text passed
    ///
    /// This is made up from the text in the `path` argument
    ///
    /// Examples:
    /// ```
    /// use std::path::PathBuf;
    /// use file_structure::File;
    ///
    /// // Makes directory "a" with directory "b" with file "c"
    /// let file = File::<()>::from_text("a/b/c");
    /// assert!(file.is_dir());
    /// assert_eq!(file.children().unwrap().len(), 1);
    /// assert_eq!(file.children().unwrap()[0].path, PathBuf::from("b"));
    /// ```
    pub fn from_text(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();

        if path.iter().count() <= 1 {
            File::new(path, FileType::Regular)
        } else {
            let mut components = path.iter();
            let (first, rest): (PathBuf, PathBuf) =
                (components.next().unwrap().into(), components.collect());

            let child = File::from_text(rest);
            File::new(first, FileType::Directory(vec![child]))
        }
    }

    /// Iterator of all `File`s in the structure
    pub fn files(&self) -> FilesIter<T> {
        FilesIter::new(self)
    }

    /// Shorthand for `self.files().paths()`, see link to `.paths()` method
    pub fn paths(&self) -> PathsIter<T> {
        self.files().paths()
    }

    /// Shorthand for unpacking `File.file_type.children()`
    pub fn children(&self) -> Option<&Vec<File<T>>> {
        self.file_type.children()
    }

    /// Shorthand for `file.file_type.is_regular()`
    pub fn is_regular(&self) -> bool {
        self.file_type.is_regular()
    }

    /// Shorthand for `file.file_type.is_dir()`
    pub fn is_dir(&self) -> bool {
        self.file_type.is_dir()
    }

    /// Shorthand for `file.file_type.is_symlink()`
    pub fn is_symlink(&self) -> bool {
        self.file_type.is_symlink()
    }
}

impl<T: fmt::Debug> fmt::Debug for File<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ds = f.debug_struct("File");
        ds.field("path", &self.path);
        ds.field("file_type", &self.file_type);
        if std::mem::size_of::<T>() != 0 {
            ds.field("extra", &self.extra);
        }
        ds.finish()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // #[should_panic]
    // fn fail_test_regular_file_with_multiple_components() {
    //     // Should use `File::from_text()` instead
    //     let _ = File::<()>::new("a/b", FileType::Regular);
    // }
}
