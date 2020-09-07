use crate::{
    error::*,
    file::{collect_files_from_current_directory, get_symlink_metadata_from_path, File},
};

use std::{
    env,
    path::{Path, PathBuf},
};

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
        let path = path.as_ref();
        if !path.exists() {
            return Err(DotaoError::NotFoundInFilesystem);
        } else if !get_symlink_metadata_from_path(&path)?.is_dir() {
            return Err(DotaoError::NotADirectory);
        }

        // Before reading the group, enter it's directory first, to access the file
        // path, now you'll need to type 'group.starting_path.join(file.path)'
        // For a `file` that is inside of `group`
        let save_previous_dir = env::current_dir().map_err(|err| DotaoError::ReadError {
            source: err,
            path: PathBuf::from("."),
        })?;

        env::set_current_dir(path).map_err(|err| DotaoError::UnableToEnterDirectory {
            source: err,
            path: path.to_path_buf(),
        })?;

        // Recursively get all chidren from the directory path
        let files = collect_files_from_current_directory(".")?;

        // Return to the directory
        env::set_current_dir(&save_previous_dir).map_err(|err| {
            DotaoError::UnableToEnterDirectory {
                source: err,
                path: save_previous_dir.to_path_buf(),
            }
        })?;

        let group = DotfileGroup::new(path.to_path_buf(), files);
        Ok(group)
    }
}
