use crate::{dotfiles::DotfileGroup, error::*, file::FileType};

use std::{
    os::unix::fs as unix_fs,
    path::{Path, PathBuf},
    process,
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

/// Used to link dotfiles
pub trait Link {
    fn link_to_home(&self, home_path: &PathBuf, link_behavior: &LinkBehavior) -> Result<u32>;
}

/// TODO: document this
// TODO: permissions checks
impl Link for DotfileGroup {
    fn link_to_home(&self, home_path: &PathBuf, link_behavior: &LinkBehavior) -> Result<u32> {
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
        let mut deque = self.deque_from_file_references();
        let mut paths_to_link: Vec<&PathBuf> = vec![];

        // Please document this im so tired right now that I can't
        while let Some(file) = deque.pop_front() {
            let target_path = home_path.join(&file.path);

            if !target_path.exists() {
                paths_to_link.push(&file.path);
            } else {
                // Type for both source and target
                let source_file_type = &file.file_type;
                let target_file_type = FileType::from_path_shallow(&target_path, false)?;
                println!(" --- {} {:?}", &target_path.display(), target_file_type);

                let can_be_deleted = can_i_delete_it(&target_path);

                use FileType::*;
                match (source_file_type, target_file_type) {
                    // Maybe this check shouldn't be here, but I wanna be 100% sure that this won't
                    // happen at this point it can't be a symlink O.o
                    (SymbolicLink { .. }, _) => {
                        panic!("This should not be here!");
                    },

                    // Won't overwrite it, for now
                    (_, File) => {
                        if link_behavior.overwrite_files {
                            todo!();
                        } else {
                            // Other than overwriting, we can check the size of the file, and then
                            // it's contents, to see if it is the exact
                            // same as the source one, if so, link
                            // if a custom option is already set yoooo

                            eprintln!("Encountered a file at {}, exiting.", target_path.display());
                            process::exit(1);
                        }
                    },

                    (_, SymbolicLink { .. }) => {
                        if link_behavior.overwrite_symbolic_links {
                            // Other than overwriting, we can check the size of the file, and then
                            // it's contents, to see if it is the exact
                            // same as the source one, if so, link
                            // if a custom option is already set yoooo

                            eprintln!("Encountered a file at {}, exiting.", target_path.display());
                            process::exit(1);
                        } else {
                        }
                    },

                    (Directory { children }, Directory { .. }) => {
                        for child in children {
                            deque.push_front(child);
                        }
                    },

                    // For now
                    _ => {
                        todo!();
                    },
                }
            }
        }

        // Check errors
        // if error...

        let mut link_counter = 0;

        if paths_to_link.len() == 0 {
            println!("Nothing to do.");
            return Ok(link_counter);
        }

        println!("{:#?}", paths_to_link);

        for path in paths_to_link {
            let source_path = self
                .starting_path
                .join(path)
                .canonicalize()
                .expect("Error resolving path");
            let target_path = home_path.join(path);
            println!("source = {}", source_path.display());
            println!("target = {}", target_path.display());
            let result = symlink_with_checks(source_path, target_path);
            link_counter += 1;
            println!("{:#?}", result);
        }

        Ok(link_counter)
    }
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
