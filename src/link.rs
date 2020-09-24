use crate::{
    dotfiles::DotfileGroup,
    error::*,
    file::{File, FileType},
};

use std::{collections::VecDeque, io, path::PathBuf};

#[derive(Debug, Default, Clone)]
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

#[derive(Default)]
pub struct LinkInformation {
    pub groups: Vec<DotfileGroup>,
    pub payload: LinkPayload,
    pub link_behavior: LinkBehavior,
    pub conflicts: Vec<Box<dyn std::error::Error + 'static>>,
}

#[derive(Default, Clone)]
pub struct LinkPayload {
    links: Vec<(PathBuf, PathBuf)>,
    deletes: Vec<(PathBuf, FileType)>,
}

impl LinkInformation {
    pub fn new() -> Self {
        LinkInformation {
            groups: vec![],
            payload: LinkPayload::default(),
            link_behavior: LinkBehavior::default(),
            conflicts: vec![],
        }
    }

    pub fn prepare_linkage_to_home(&mut self, home_path: &PathBuf) -> Result<()> {
        let copy = self.groups.clone();
        for group in copy.into_iter() {
            //
            let mut deque: VecDeque<File> = VecDeque::from(group.files.clone());
            // println!("{:#?}", deque);

            while let Some(file) = deque.pop_front() {
                // We have source and destination

                let source_path = group.starting_path.join(&file.path);
                let destination_path = home_path.join(&file.path);

                let source_file_type = file.file_type;
                let destination_file_type = FileType::from_path_shallow(&destination_path, false)?;

                if let FileType::SymbolicLink { .. } = source_file_type {
                    panic!("No symlinks allowed in source");
                }

                if !source_path.exists() {
                    panic!("This should exist");
                }

                // Ok
                if !destination_path.exists() {
                    let link_payload = (source_path, destination_path);
                    self.payload.links.push(link_payload);
                    continue;
                }

                match destination_file_type {
                    FileType::File => {
                        self.link_check_for_regular_file(
                            source_path,
                            destination_path,
                            source_file_type,
                            destination_file_type,
                        )
                        .unwrap();
                    },
                    FileType::Directory { children } => {
                        let should_add_children =
                            self.link_check_for_directory(source_path, destination_path);

                        if should_add_children.unwrap() {
                            for child in children.clone().into_iter() {
                                deque.push_back(child);
                            }
                        }
                    },
                    FileType::SymbolicLink { target_path } => {
                        self.link_check_for_symlink(
                            source_path,
                            destination_path,
                            source_file_type,
                            &target_path,
                        )
                        .unwrap();
                    },
                }
            }
        }
        Ok(())
    }

    fn link_check_for_regular_file(
        &mut self,
        source_path: PathBuf,
        destination_path: PathBuf,
        _source_file_type: FileType,
        destination_file_type: FileType,
    ) -> io::Result<()> {
        if self.link_behavior.overwrite_files {
            println!("Deleting file at '{}'?", destination_path.display());
            self.payload
                .deletes
                .push((destination_path.clone(), destination_file_type));
            self.payload.links.push((source_path, destination_path));
        } else {
            println!("Error deleting file at '{}'?", destination_path.display());
            self.conflicts.push(Box::new(DotaoError::LinkError2 {
                source_path,
                destination_path,
            }))
        }
        Ok(())
    }

    fn link_check_for_directory(
        &mut self,
        source_path: PathBuf,
        destination_path: PathBuf,
        // _source_file_type: FileType,
    ) -> io::Result<bool> {
        /* if self.link_behavior.overwrite_directories */
        if false {
            println!("Deleting directory at '{}'?", destination_path.display());
            self.payload
                .deletes
                .push((destination_path.clone(), FileType::Directory {
                    children: vec![],
                }));
            self.payload.links.push((source_path, destination_path));
            return Ok(false);
        } else {
            println!(
                "Error deleting directory at '{}'?",
                destination_path.display()
            );
            self.conflicts.push(Box::new(DotaoError::LinkError2 {
                source_path,
                destination_path,
            }))
        }
        Ok(true)
    }

    fn link_check_for_symlink(
        &mut self,
        source_path: PathBuf,
        destination_path: PathBuf,
        destination_file_type: FileType,
        current_path: &PathBuf,
    ) -> io::Result<()> {
        if *current_path == source_path {
            return Ok(()); // Nothing to do, already linked
        } else if self.link_behavior.overwrite_symbolic_links {
            self.payload
                .deletes
                .push((destination_path.clone(), destination_file_type));
            self.payload.links.push((source_path, destination_path));
        } else {
            panic!("Cannot overwrite symlink");
        }

        Ok(())
    }

    pub fn is_ready(&self) -> bool {
        for (i, error) in self.conflicts.iter().enumerate() {
            println!("{} - {:#?}", i, error);
        }

        self.conflicts.is_empty()
    }

    pub fn configure_behavior(&mut self, link_behavior: LinkBehavior) {
        self.link_behavior = link_behavior;
    }

    pub fn add_groups(&mut self, mut groups: Vec<DotfileGroup>) {
        self.groups.append(&mut groups);
    }
}
