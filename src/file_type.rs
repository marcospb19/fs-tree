use crate::{
    error::*,
    file::File,
    util::{collect_directory_children, fs_filetype_from_path, symlink_target},
};

use std::{
    fmt,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FileType<T: Default> {
    Regular,
    Directory(Vec<File<T>>),
    Symlink(PathBuf),
}

impl<T: Default> FileType<T> {
    /// Recursively creates `FileType` from path.
    ///
    /// # Example
    /// ```
    /// use file_structure::{FileType, FSError};
    ///
    /// fn main() -> Result<(), FSError> {
    ///     let file_type = FileType::from_path("src/", true)?;
    ///
    ///     if let FileType::Directory(ref children) = file_type {
    ///         println!("We found {} files!", children.len()); // vec.len()
    ///
    ///         for child in children {
    ///             println!("{:#?}", child);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Useful when
    ///
    /// # Notes:
    ///
    /// If `follow_symlinks` is `true`, when gathering information about the
    /// file type, this function will make a system call that
    /// traverses paths until there is no `symlink` left, this means that the
    /// return type in this case can never be the variant
    /// `FileType::Symlink(_)`, if you want to read from the path and also check
    /// if it is a `symlink`, set `follow_symlinks` to `false`.
    ///
    /// For each directory, call the function recursively.
    ///
    /// See also `from_path_shallow`.
    pub fn from_path(path: impl AsRef<Path>, follow_symlinks: bool) -> FSResult<Self> {
        // Reuse code from `from_path_shallow`
        //
        // If FileType::Directory, populate with it's children, else, do nothing
        let result = match FileType::from_path_shallow(&path, follow_symlinks)? {
            FileType::Directory(_) => {
                FileType::Directory(collect_directory_children(&path, follow_symlinks)?)
            },
            other => other,
        };

        Ok(result)
    }

    /// Similar to `from_path`, but leaves `Directory` and `Symlink` empty.
    ///
    /// This function is guaranteed to only make one syscall for the `FileType`,
    /// this means that it cannot read all the elements from inside of the
    /// directories.
    ///
    /// This is useful when you want to make a quick check on a file type
    /// without going into it's thousand subsequent files, that would take a lot
    /// of time.
    ///
    /// # Example:
    /// ```
    /// use file_structure::{FileType, FSError};
    ///
    /// fn main() -> Result<(), FSError> {
    ///     let file_type = FileType::from_path_shallow("/sbin", true)?;
    ///
    ///     if !file_type.is_dir() {
    ///         println!("There's something wrong with our file system.");
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn from_path_shallow(path: impl AsRef<Path>, follow_symlink: bool) -> FSResult<Self> {
        let fs_file_type = fs_filetype_from_path(&path, follow_symlink)?;

        // From the `fs::FileType` check if it is regular file, directory, or symlink
        let result = {
            if fs_file_type.is_file() {
                FileType::Regular
            } else if fs_file_type.is_dir() {
                FileType::Directory(vec![])
            } else if fs_file_type.is_symlink() {
                FileType::Symlink(symlink_target::<T, &Path>(path.as_ref())?)
            } else {
                todo!("Other file types.")
            }
        };
        Ok(result)
    }

    /// Checks variant `FileType::Regular(_)`
    pub fn is_regular(&self) -> bool {
        matches!(self, FileType::Regular)
    }

    /// Checks variant `FileType::Directory(_)`
    pub fn is_dir(&self) -> bool {
        matches!(self, FileType::Directory(_))
    }

    /// Checks variant `FileType::Symlink(_)`
    pub fn is_symlink(&self) -> bool {
        matches!(self, FileType::Symlink(_))
    }

    /// Shorthand for unpacking `FileType::Directory(ref children)`
    pub fn children(&self) -> Option<&Vec<File<T>>> {
        match self {
            FileType::Directory(ref children) => Some(children),
            _ => None,
        }
    }
}

impl<T: Default> Default for FileType<T> {
    fn default() -> Self {
        Self::Regular
    }
}

impl<T: Default> fmt::Display for FileType<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileType::Regular => write!(f, "file"),
            FileType::Directory(_) => write!(f, "directory"),
            FileType::Symlink(_) => write!(f, "symbolic link"),
        }
    }
}
