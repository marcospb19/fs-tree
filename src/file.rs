use crate::{
    error::*,
    file_type::FileType,
    iter::{FilesIter, PathsIter},
};

use std::path::{Path, PathBuf};

/// Recursive file representation that supports a generic extra field
///
/// For now, all paths are made of multiple components, for example:
///
/// ```txt
/// "a": [
///     "b",
///     "c"
/// ]
/// ```
///
/// The inner files path is "a/b" and "a/c" instead of just "b" or "c"
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        assert_eq!(1, path.as_ref().components().count(), "Only one component");

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
    pub fn new_from_path(path: impl AsRef<Path>, follow_symlinks: bool) -> FsResult<Self> {
        let file_type = FileType::from_path(&path, follow_symlinks)?;
        let result = File::new(path, file_type);

        Ok(result)
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
}

// impl<T: Default> Default for File<T> {
//     fn default() -> Self {
//         File {
//             path: Default::default(),
//             file_type: FileType::Regular,
//             extra: Default::default(),
//         }
//     }
// }
