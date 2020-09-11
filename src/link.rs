use crate::{dotfiles::DotfileGroup, error::*, file::File};
use std::{collections::VecDeque, path::PathBuf};

trait Link {
    fn link_to_home(&self, home_path: &PathBuf) -> Result<()>;
}

impl Link for DotfileGroup {
    fn link_to_home(&self, _home_path: &PathBuf) -> Result<()> {
        let result = {
            // Directories go to the front, files to the end, consume front
            // recursively until order results looks like DFS order
            let mut deque = VecDeque::<&File>::new();
            let _vec = VecDeque::<&File>::new();

            for file in &self.files {
                if file.file_type.is_directory() {
                    deque.push_front(&file);
                } else {
                    deque.push_back(&file);
                }
            }
            Ok(())
        };

        result.map_err(|err| DotaoError::LinkError(err))
    }
}
