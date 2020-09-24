use crate::{
    dotfiles::DotfileGroup,
    file::{File, FileType},
};

use std::{io, path::PathBuf};

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

#[derive(Default)]
pub struct LinkInformation {
    pub groups: Vec<DotfileGroup>,
    pub files_to_delete: Vec<File>,
    pub files_to_link: Vec<File>,
    pub link_behavior: LinkBehavior,
    pub errors: Vec<Box<dyn std::error::Error + 'static>>,
}

impl LinkInformation {
    pub fn new() -> Self {
        LinkInformation {
            groups: vec![],
            files_to_delete: vec![],
            files_to_link: vec![],
            link_behavior: LinkBehavior::default(),
            errors: vec![],
        }
    }

    pub fn prepare_linkage_to_home(&self, _home_path: &PathBuf) {
        for group in &self.groups {
            println!("{:#?}", group);
            // link_information
            //     .prepare_to_link(&group, &home_path)
            //     .unwrap_or_else(|err| {
            //         eprintln!(
            //             "Error trying to prepare linkage of group {:#?}: {}",
            //             group, err
            //         );
            //     });
        }
    }

    pub fn is_ready(&self) -> bool {
        false
    }

    pub fn configure_behavior(&mut self, link_behavior: LinkBehavior) {
        self.link_behavior = link_behavior;
    }

    pub fn add_groups(&mut self, mut groups: Vec<DotfileGroup>) {
        self.groups.append(&mut groups);
    }

    fn _link_check_for_regular_file(
        _link_information: &mut LinkInformation,
        _target_file_type: FileType,
    ) -> io::Result<bool> {
        Ok(false)
    }

    fn _link_check_for_directory(
        _link_information: &mut LinkInformation,
        _target_file_type: FileType,
    ) -> io::Result<bool> {
        Ok(false)
    }
}
