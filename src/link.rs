use crate::{
    dotfiles::DotfileGroup,
    error::*,
    file::{File, FileType},
};

use std::{collections::VecDeque, path::PathBuf};

pub trait Link {
    fn link_to_home(&self, home_path: &PathBuf) -> Result<()>;
}

impl Link for DotfileGroup {
    fn link_to_home(&self, home_path: &PathBuf) -> Result<()> {
        // Directories go to the front, files to the end, consume front
        // recursively until order results looks like DFS order
        let mut deque = VecDeque::<&File>::new();
        let mut vec = VecDeque::<&File>::new();

        for file in &self.files {
            if file.file_type.is_directory() {
                deque.push_front(&file);
            } else {
                deque.push_back(&file);
            }
        }

        // Please document this im so tired right now that I can't
        while let Some(file) = deque.pop_front() {
            // Get real relative path based on home_path
            let target_path = home_path.join(&file.path);

            if target_path.exists() {
                // Don't follow symlinks, we need to link, so detect them
                let target_file_type = FileType::from_path_shallow(target_path, false)?;

                match target_file_type {
                    FileType::File => {
                        eprintln!("File here! do you want to delete it or what?");
                    },
                    FileType::Directory { children: _ } => {},
                    FileType::SymbolicLink { target_path: _ } => {},
                }
                if target_file_type.is_directory() {
                    unimplemented!();
                }
                if let FileType::Directory { children: _ } = &file.file_type {
                } else if let FileType::File = file.file_type {
                    // unimplemented!();
                } else {
                    panic!("panicou kaka panicao");
                }
            } else {
                vec.push_back(file);
                if let FileType::Directory { children } = &file.file_type {
                    for child in children {
                        deque.push_back(&child);
                    }
                }
                // FINE!
            }
        }

        //
        Ok(())
    }
}
