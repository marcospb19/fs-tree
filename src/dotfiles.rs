use crate::{
    error::*,
    file::{collect_files_from_current_directory, File, FileType},
};

use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct DotfileGroup {
    pub starting_path: PathBuf,
    pub files: Vec<File>,
}

impl DotfileGroup {
    pub fn new(starting_path: PathBuf, files: Vec<File>) -> Self {
        DotfileGroup {
            starting_path,
            files,
        }
    }

    pub fn from_directory_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            return Err(DotaoError::NotFoundInFilesystem);
        } else if !FileType::from_path_shallow(&path)?.is_directory() {
            return Err(DotaoError::NotADirectory);
        }

        // Recursively get all chidren from the directory path
        let files = collect_files_from_current_directory(&path)?;

        let mut group = DotfileGroup::new(path, files);
        group.trim_starting_path_from_files();
        Ok(group)
    }

    pub fn trim_starting_path_from_files(&mut self) {
        let get_len_of_pathbuf = |path: &PathBuf| -> usize {
            let mut len = 0;
            for _ in path {
                len += 1;
            }
            len
        };

        // Calculate length of PathBuf iterator
        let len_to_trim = get_len_of_pathbuf(&self.starting_path);

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
}
