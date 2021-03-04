use std::{
    fs, io,
    path::{Path, PathBuf},
};

use file_type_enum::FileType as FileTypeEnum;
use tsml::FileTree;

use crate::{
    error,
    util::{self, to_utf},
};

#[derive(Debug, Default, Clone)]
pub struct StatusDiff<'a> {
    // Ok stuff
    pub linked_correctly: Vec<&'a FileTree>,
    pub ready_to_link: Vec<(&'a FileTree, PathBuf)>,
    // Error stuff
    pub missing_source: Vec<&'a FileTree>,
    pub linked_incorrectly: Vec<(&'a FileTree, PathBuf)>,
    pub not_a_symlink: Vec<(&'a FileTree, FileTypeEnum)>,
    pub no_permission_to_link_to_target: Vec<&'a FileTree>,
}

impl<'a> StatusDiff<'a> {
    fn new() -> Self {
        Self::default()
    }

    pub fn from_groups_map(map: &'a tsml::GroupsMap) -> Self {
        let mut diff = Self::new();

        let vecs = map.values().flat_map(|x| x.iter());
        let groups = map.keys();
        let zipped = vecs.zip(groups);
        // let zipped_trees = zipped_tree_vecs
        // .flat_map(|(vecs, groups)| vecs.files().skip_dirs(true).map(move |x| (x, groups)));

        for (tree, group_name) in zipped {
            for file in tree.files().skip_dirs(true) {
                if !file.is_regular() {
                    error!("Your config contains a symlink syntax that is not yet supported, to the file '{}'.", to_utf(file.path()));
                }

                let source_location = Path::new(group_name).join(file.path());
                // dbg!(&source_location);
                if !source_location.exists() {
                    diff.missing_source.push(file);
                    continue;
                }

                let backup_dir = util::backup_dir();
                let destination_location = backup_dir.join(file.path());
                if !destination_location.exists() {
                    diff.ready_to_link.push((file, PathBuf::from(group_name)));
                    continue;
                }

                // Gather file_type, treat ErrorKind::PermissionDenied, or exit
                let file_type = match FileTypeEnum::from_symlink_path(&destination_location) {
                    Ok(file_type) => file_type,
                    Err(err) => {
                        if matches!(err.kind(), io::ErrorKind::PermissionDenied) {
                            diff.no_permission_to_link_to_target.push(file);
                            continue;
                        } else {
                            error!(
                                "Error: interrupted while scanning HOME searching for file at '{}': {}.",
                                to_utf(destination_location),
                                err
                            );
                        }
                    },
                };

                let _target_path = match file_type {
                    FileTypeEnum::Symlink => {
                        fs::read_link(&destination_location)
                            .unwrap_or_else(|err| error!("asjdaskdasjdn: {}.", err));
                    },
                    _ => {
                        diff.not_a_symlink.push((file, file_type));
                        continue;
                    },
                };

                //
                //
                //
            }
        }
        //     // If it is error, treat

        //     // Match on success of FileTypeEnum::from_symlink_path query
        //     match file_type {
        //         FileTypeEnum::Symlink => {
        //             let link_target = fs::read_link(&dotfiles_path).unwrap_or_else(|err| {
        //                 error!(
        //                     "Error while trying to read symlink at '{}': {}.",
        //                     to_utf(&dotfiles_path),
        //                     err
        //                 )
        //             });
        //             // If it's pointing to the right place
        //             if link_target == expected_path {
        //                 diff.linked_correctly.push(file);
        //             } else {
        //                 diff.linked_incorrectly.push((file, link_target));
        //             }
        //         },
        //         _ => diff.different_file_type_at_location.push((file, file_type)),
        //     }
        // }
        // dbg!(&diff);
        diff
    }

    pub fn is_clear(&self) -> bool {
        self.missing_source.is_empty()
            && self.linked_incorrectly.is_empty()
            && self.not_a_symlink.is_empty()
            && self.no_permission_to_link_to_target.is_empty()
    }
}
