use crate::{
    dotfiles::DotfileGroup,
    error::*,
    file::{File, FileType},
};

use std::{collections::VecDeque, os::unix::fs as unix_fs, path::PathBuf, process};

pub trait Link {
    fn link_to_home(&self, home_path: &PathBuf) -> Result<()>;
}

// TODO: permissions checks
impl Link for DotfileGroup {
    fn link_to_home(&self, home_path: &PathBuf) -> Result<()> {
        // For this code we'll create a deque and a vec
        //
        // The deque will contain files left to check and pass to the vec
        //
        // The vec contains itens that are ok for linking, checked and OK to proceed
        //
        // The order of the deque is weirdly specific, like an DFS traversal that
        // sometimes prioritizes files over directories, the intent here is to make the
        // error messages more intuitive
        let mut deque = VecDeque::<&File>::new();
        let mut paths_to_link: Vec<&PathBuf> = vec![];

        for file in &self.files {
            if file.file_type.is_directory() {
                deque.push_back(&file);
            } else {
                deque.push_front(&file);
            }
        }

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

                use FileType::*;
                match (source_file_type, target_file_type) {
                    // Maybe this check shouldn't be here, but I wanna be 100% sure that this won't
                    // happen at this point it can't be a symlink O.o
                    (SymbolicLink { .. }, _) => {
                        todo!();
                    },

                    // Won't overwrite it, for now
                    (_, File) => {
                        // Other than overwriting, we can check the size of the file, and then it's
                        // contents, to see if it is the exact same as the source one, if so, link
                        // if a custom option is already set yoooo

                        eprintln!("Encountered a file at {}, exiting.", target_path.display());
                        process::exit(1);
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

        if paths_to_link.len() == 0 {
            println!("Nothing to do.");
            return Ok(());
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
            let result = unix_fs::symlink(source_path, target_path);
            println!("{:#?}", result);
        }

        Ok(())
    }
}
