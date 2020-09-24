// In this module we find all the implementation of the linking features
//
// We have LinkInformation, that keeps the steps needed to perform the linkage,
// functions fill this struct until it's ready to be sent to the function that
// actually links everything.
#[allow(unused_imports)]
use crate::{
    dotfiles::DotfileGroup,
    error::*,
    file::{File, FileType},
};

#[allow(unused_imports)]
use permissions::is_file_removable;

#[allow(unused_imports)]
use std::{
    io,
    os::unix::fs as unix_fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Default)]
pub struct LinkBehavior {
    pub overwrite_files: bool,
    pub overwrite_symbolic_links: bool,
}

impl LinkBehavior {
    pub fn new(overwrite_files: bool, overwrite_symbolic_links: bool) -> Self {
        LinkBehavior {
            overwrite_files,
            overwrite_symbolic_links,
        }
    }
}

struct LinkInformation<'a> {
    pub files_to_delete: Vec<&'a File>,
    pub files_to_link: Vec<&'a File>,
}

impl LinkInformation<'_> {
    pub fn new() -> Self {
        LinkInformation {
            files_to_delete: vec![],
            files_to_link: vec![],
        }
    }
}

pub fn link_to_home(
    _dotfile_group: &DotfileGroup,
    _home_path: &PathBuf,
    _link_behavior: &LinkBehavior,
) -> Result<()> {
    Ok(())
}

fn link_check_for_regular_file(
    _link_information: &mut LinkInformation,
    _target_file_type: FileType,
) -> io::Result<bool> {
    Ok(false)
}

fn link_check_for_directory(
    _link_information: &mut LinkInformation,
    _target_file_type: FileType,
) -> io::Result<bool> {
    Ok(false)
}

// /// Wrap std::os::unix::fs::symlink with Dotao's Result<()>, extra checks
// pub fn symlink_with_checks(src: &impl AsRef<Path>, dest: &impl AsRef<Path>)
// -> Result<()> {     let (src, dest) = (src.as_ref(), dest.as_ref());
//     if !src.exists() {
//         return Err(DotaoError::NotFoundInFilesystem);
//     } else if true {
//         // Check if dest.exists()!!!!, overwrite???? vixe!
//         // Check permissions?
//     }

//     unix_fs::symlink(src, dest).map_err(|source| DotaoError::LinkError {
//         from: src.to_path_buf(),
//         to: dest.to_path_buf(),
//         source,
//     })
// }
