use crate::{
    error::*,
    file::{collect_files_from_current_directory, File, FileType},
};

use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

/// `DotfileGroup` represents a folder with a list of files (dotfiles) inside of
/// it.
///
/// These dotfiles can be files, or directories, and each directory is a tree,
/// here is a example with `i3`.
///
/// ```sh
/// cd ~/dotfiles
/// dotao i3/
/// ```
///
/// When the user types this in the terminal, we'll create:
/// ```rust
/// DotfileGroup {
///     starting_path: "i3",
///     files: vec![],
/// }
/// ```
///
/// Then the files will be filled with everything that is inside of the i3
/// folder, recursively, following symlinks
///
/// So, the method that links the DotfileGroup will panic if you let a
/// FileType::SymbolicLink inside of the file tree, this may change in the
/// future, ok?
///
/// For now just keep in mind that it does not make sense to have a symlink
/// there
#[derive(Debug, Default)]
pub struct DotfileGroup {
    pub starting_path: PathBuf,
    pub files: Vec<File>,
}

impl DotfileGroup {
    /// TODO: document this
    pub fn new(starting_path: PathBuf, files: Vec<File>) -> Self {
        DotfileGroup {
            starting_path,
            files,
        }
    }

    /// TODO: document this
    pub fn from_directory_path(path: impl AsRef<Path>, follow_symlinks: bool) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        if !path.exists() {
            return Err(DotaoError::NotFoundInFilesystem);
        } else if !FileType::from_path_shallow(&path, follow_symlinks)?.is_directory() {
            return Err(DotaoError::NotADirectory);
        }

        // Recursively get all chidren from the directory path
        let files = collect_files_from_current_directory(&path, follow_symlinks)?;
        let mut group = DotfileGroup::new(path, files);

        // Adjust all path for file tree, in a way that, considering a file from it:
        // starting_path.join(file).exists()
        group.trim_starting_path_from_files();
        Ok(group)
    }

    /// TODO: document this
    pub fn trim_starting_path_from_files(&mut self) {
        // Calculate length of PathBuf iterator
        let len_to_trim = self.starting_path.iter().count();

        let mut stack: Vec<&mut File> = vec![];
        for file in self.files.as_mut_slice() {
            stack.push(file);
        }

        // Use a stack to trim all files recursively
        while let Some(file) = stack.pop() {
            // Trim file.path
            file.path = file.path.into_iter().skip(len_to_trim).collect();

            // If it is a directory, push the other files too
            if let FileType::Directory { children } = &mut file.file_type {
                for child in children {
                    stack.push(child);
                }
            }
        }
    }

    /// From DotfileGroup.files (Vec<File>) to Deque<&File>
    /// WHy this order? marcospb19?
    pub fn deque_from_file_references(&self) -> VecDeque<&File> {
        let mut deque = VecDeque::new();

        for file in &self.files {
            if file.file_type.is_directory() {
                deque.push_back(file);
            } else {
                deque.push_front(file);
            }
        }

        deque
    }
}
