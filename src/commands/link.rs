use std::{
    iter,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

use crate::{
    diff::StatusDiff,
    error,
    util::{self, to_utf},
};

pub fn construct_link_target(file_path: impl AsRef<Path>, group_path: impl AsRef<Path>) -> PathBuf {
    let directory_level = file_path.as_ref().components().count();
    let mut path = iter::repeat("..").take(directory_level - 1).collect::<PathBuf>(); // Subtraction could fail
    path.push("dotfiles");
    path.push(group_path);
    path.push(file_path);
    path
}

// Link everything to the backup_dir
pub fn run_link_command() {
    let groups = tsml::Groups::from_path("dotao.tsml").unwrap();

    let diff = StatusDiff::from_groups_map(&groups.map);
    let can_link = diff.is_clear();

    for _ in diff.linked_incorrectly.iter() {
        eprintln!("Err: There is already a link at 'lilili', but it links to another file:");
        eprintln!("    found 'lalal', instead of: 'lele'.");
        // eprintln!("    found '{}', instead of: '{}'.", );
    }

    for x in diff.no_permission_to_link_to_target.iter() {
        eprintln!("Permission denied to apply links to '{}'.", to_utf(x.path()));
    }

    for _ in diff.not_a_symlink.iter() {
        // Another file, what file?
    }

    if !can_link {
        error!("Aborting.");
    }

    if diff.ready_to_link.is_empty() {
        println!("nothing to link, ok!");
        return;
    }

    for (file, group_name) in diff.ready_to_link.iter() {
        let backup_dir = util::backup_dir();
        let destination_location = backup_dir.join(file.path());

        let link_relative_target_path = construct_link_target(file.path(), group_name);
        symlink(link_relative_target_path, destination_location).unwrap();
    }
    //     match file {
    //         tsml::FileTree::Regular { .. } => {
    //             symlink(&dotfiles_path, &backup_dir_file_path).unwrap_or_else(|err| {
    //                 error!(
    //                     "Error while trying to make link for regular file '{}' -> '{}': {}",
    //                     dotfiles_path.display(),
    //                     home_path.display(),
    //                     err
    //                 )
    //             });
    //         },
    //         tsml::FileTree::Directory { children, .. } => {
    //             // Verify if should link or create directory
    //             // If there's no children, link, else, create directory
    //             if children.is_empty() {
    //                 symlink(&dotfiles_path, &home_path).unwrap_or_else(|err| {
    //                     error!(
    //                         "Error while trying to make link for directory '{}' -> '{}': {}",
    //                         dotfiles_path.display(),
    //                         home_path.display(),
    //                         err
    //                     );
    //                 });
    //             } else {
    //                 fs::create_dir_all(&home_path)
    //                     .expect("Error while trying to create directory.");
    //             }
    //         },
    //         tsml::FileTree::Symlink { .. } => todo!(),
    //     }
    // }
}
