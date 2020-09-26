use crate::{
    dotfiles::DotfileGroup,
    error::*,
    file::{File, FileType},
};

use std::{
    collections::VecDeque,
    io,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Clone)]
pub struct LinkBehavior {
    pub interactive_mode: bool,
    pub overwrite_files: bool,
    pub overwrite_directories: bool,
    pub overwrite_symbolic_links: bool,
}

impl LinkBehavior {
    pub fn new(
        interactive_mode: bool,
        overwrite_files: bool,
        overwrite_directories: bool,
        overwrite_symbolic_links: bool,
    ) -> Self {
        LinkBehavior {
            interactive_mode,
            overwrite_files,
            overwrite_directories,
            overwrite_symbolic_links,
        }
    }
}

#[derive(Default, Debug)]
pub struct LinkInformation {
    pub groups: Vec<DotfileGroup>,
    pub payload: LinkPayload,
    pub link_behavior: LinkBehavior,
    pub errors: Vec<Box<dyn std::error::Error + 'static>>,
    pub conflicts: Vec<Box<dyn std::error::Error + 'static>>,
}

#[derive(Default, Clone, Debug)]
pub struct LinkPayload {
    links: Vec<(PathBuf, PathBuf)>,
    deletes: Vec<(PathBuf, FileType)>,
}

impl LinkInformation {
    pub fn new() -> Self {
        LinkInformation {
            errors: vec![],
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

                // Symlink source_path needs to be absolute
                let source_path = group.starting_path.join(&file.path).canonicalize()?;
                let destination_path = home_path.join(&file.path);

                if !destination_path.exists() {
                    self.payload
                        .links
                        .push((source_path.clone(), destination_path.clone()));
                    continue;
                }

                let source_file_type = file.file_type;
                let destination_file_type = FileType::from_path_shallow(&destination_path, false)?;

                if let FileType::SymbolicLink { .. } = source_file_type {
                    panic!("No symlinks allowed in source");
                }

                if !source_path.exists() {
                    panic!("This should exist");
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
                file_type: FileType::from_path_shallow(&destination_path, false).unwrap(),
                source_path,
                destination_path,
            }))
        }
        Ok(())
    }

    #[allow(unreachable_code)]
    fn link_check_for_directory(
        &mut self,
        source_path: PathBuf,
        destination_path: PathBuf,
        // _source_file_type: FileType,
    ) -> io::Result<bool> {
        /* if self.link_behavior.overwrite_directories */
        println!("Deleting directory at '{}'?", destination_path.display());
        self.payload
            .deletes
            .push((destination_path.clone(), FileType::Directory {
                children: vec![],
            }));
        self.payload.links.push((source_path, destination_path));
        unimplemented!();
        Ok(true)

        // if false {
        //     println!("Deleting directory at '{}'?",
        // destination_path.display());     self.payload
        //         .deletes
        //         .push((destination_path.clone(), FileType::Directory {
        //             children: vec![],
        //         }));
        //     self.payload.links.push((source_path, destination_path));
        //     return Ok(false);
        // } else {
        //     println!(
        //         "Error deleting directory at '{}'?",
        //         destination_path.display()
        //     );
        //     self.conflicts.push(Box::new(DotaoError::LinkError2 {
        //         file_type: FileType::from_path_shallow(&destination_path,
        // false).unwrap(),         source_path,
        //         destination_path,
        //     }))
        // }
        // Ok(true)
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

    pub fn configure_behavior(&mut self, link_behavior: LinkBehavior) {
        self.link_behavior = link_behavior;
    }

    pub fn add_group(&mut self, group: DotfileGroup) {
        self.groups.push(group);
    }

    pub fn critical_error_occurred(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn show_errors(&self) {
        eprintln!("List of errors:");
        for error in &self.errors {
            eprintln!("{:#?}", error);
        }
    }

    pub fn proceed_and_link(&self) -> Result<()> {
        for (source, dest) in &self.payload.links {
            symlink_with_checks(source, dest)?;
        }

        Ok(())
    }
}

/// Wrap std::os::unix::fs::symlink with Dotao's Result<()>, extra checks
pub fn symlink_with_checks(src: &impl AsRef<Path>, dest: &impl AsRef<Path>) -> Result<()> {
    let (src, dest) = (src.as_ref(), dest.as_ref());
    if !src.exists() {
        return Err(DotaoError::NotFoundInFilesystem);
    } else if true {
        // Check if dest.exists()!!!!, overwrite???? vixe!
        // Check permissions?
    }

    symlink(src, dest).map_err(|source| DotaoError::LinkError {
        source_path: src.to_path_buf(),
        destination_path: dest.to_path_buf(),
        source,
    })
}
