use crate::{
    dotfiles::DotfileGroup,
    error::*,
    file::{File, FileType},
    util::can_i_delete_it,
};

use std::{
    io,
    os::unix::fs as unix_fs,
    path::{Path, PathBuf},
};

/// TODO: document this
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

/// Used to link dotfiles
// pub trait Link {
//     fn link_to_home(&self, home_path: &PathBuf, link_behavior: &LinkBehavior) -> Result<u32>;
// }

#[allow(unused_variables, unused_mut, unreachable_patterns)]
/// TODO: document this
// TODO: permissions checks
// impl Link for DotfileGroup {
pub fn link_to_home(
    dotfile_group: &DotfileGroup,
    home_path: &PathBuf,
    link_behavior: &LinkBehavior,
) -> Result<u32> {
    // We'll create a deque and vec of &File, that we'll use to check each file
    // entry.
    //
    // The deque will contain files left to check and pass to the vec
    //
    // The vec contains itens that are ok for linking, checked and OK to proceed
    //
    // The order of the deque is weirdly specific, like an DFS traversal that
    // sometimes prioritizes files over directories, the intent here is to make the
    // error messages more intuitive
    let mut deque = dotfile_group.deque_from_file_references();
    let mut link_information = LinkInformation::new();

    // Please document this im so tired right now that I can't
    while let Some(file) = deque.pop_front() {
        let target_path = home_path.join(&file.path);
        let source_path = dotfile_group
            .starting_path
            .join(&file.path)
            .canonicalize()
            .expect("Error resolving path of file, should not fail!");

        if !target_path.exists() {
            link_information.files_to_link.push(&file);
            continue;
        }

        // Type for both source and target
        let source_file_type = &file.file_type;
        let target_file_type = FileType::from_path_shallow(&target_path, false)?;
        println!(" --- {} {:?}", &target_path.display(), target_file_type);

        // let files_to_delete = vec![];
        let can_be_deleted =
            can_i_delete_it(&target_path).map_err(|err| DotaoError::LinkError {
                from: source_path,
                to: target_path,
                source: err,
            })?;

        use FileType::*;

        if let SymbolicLink { target_path } = target_file_type {
            unimplemented!();
        }

        match source_file_type {
            // Maybe this check shouldn't be here, but I wanna be 100% sure that this won't
            // happen at this point it can't be a symlink O.o
            SymbolicLink { .. } => {
                panic!("DevErr: We shouldn't be trying to create symlinks of symlinks!");
            },

            File => {
                let result = link_check_for_regular_file(&mut link_information, target_file_type)
                    .expect("expect");
                continue;
            },

            Directory { .. } => {
                let result = link_check_for_directory(&mut link_information, target_file_type)
                    .expect("askndnajndjasd");
                 continue;
            },
            // (_, SymbolicLink { target_path }) => {
            //     if true {
            //         // if temp_target == source_path {
            //         continue; // Ok, already in place
            //     } else {
            //         // Now we need to deal with it
            //         if link_behavior.overwrite_symbolic_links {
            //             // files_to_delete.push(&file);
            //         } else {
            //             eprintln!(
            //                 "Problem, we found this symlink, but it points to another place: '{}' \
            //                  -> '{}'",
            //                 target_path.display(),
            //                 target_path /* temp_target */
            //                     .display()
            //             );
            //         }
            //     }
            //     if link_behavior.overwrite_symbolic_links {}
            // },

            /* Other than overwriting, we can check the size of the file, and then
             * it's contents, to see if it is the exact
             * same as the source one, if so, link
             * if a custom option is already set yoooo */

            /*         eprintln!("Encountered a file at {}, exiting.",
             * target_path.display());         process::exit(1);
             *     } else {
             *     }
             * }, */

            /* (_, File) => {
             *     // Other than overwriting, we can check the size of the file, and then
             *     // it's contents, to see if it is the exact
             *     // same as the source one, if so, link
             *     // if a custom option is already set yoooo */

            /*     eprintln!("Encountered a file at {}, exiting.", target_path.display());
             *     process::exit(1);
             * }, */

            /* (Directory { .. }, File) => {
             *     eprintln!(
             *         "Found file instead of directory: '{}'",
             *         target_path.display()
             *     );
             * }, */
        } // match
    } // for

    // Check errors
    // if error...

    let mut link_counter = 0;

    // if paths_to_link.len() == 0 {
    //     println!("Nothing to do.");
    //     return Ok(link_counter);
    // }

    // println!("{:#?}", paths_to_link);

    // for path in paths_to_link {
    //     let source_path = dotfile_group
    //         .starting_path
    //         .join(path)
    //         .canonicalize()
    //         .expect("Error resolving path");
    //     let target_path = home_path.join(path);
    //     println!("source = {}", source_path.display());
    //     println!("target = {}", target_path.display());
    //     let result = symlink_with_checks(source_path, target_path);
    //     link_counter += 1;
    //     println!("{:#?}", result);
    // }

    Ok(link_counter)
}
// }

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

/// TODO: document this
/// Wrap std::os::unix::fs::symlink with Dotao's Result<()>, extra checks
pub fn symlink_with_checks(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> Result<()> {
    let (src, dest) = (src.as_ref(), dest.as_ref());
    if !src.exists() {
        return Err(DotaoError::NotFoundInFilesystem);
    } else if true {
        // Check if dest.exists()!!!!, overwrite???? vixe!
        // Check permissions?
    }

    unix_fs::symlink(src, dest).map_err(|source| DotaoError::LinkError {
        from: src.to_path_buf(),
        to: dest.to_path_buf(),
        source,
    })
}
