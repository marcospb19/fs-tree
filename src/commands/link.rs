use std::{env, fs, os::unix::fs::symlink, path::Path};

use crate::error;

pub fn run_link_command() {
    let home = env::var_os("HOME").expect("No home detected.");
    let groups = tsml::Groups::from_path("dotao.tsml").unwrap();

    for (group_name, tree_vec) in groups.map.iter() {
        for tree in tree_vec.iter() {
            for file in tree.files() {
                let semi_path = file.path();
                let dotfiles_path = Path::new(group_name).join(&semi_path);
                let dotfiles_path = dotfiles_path.canonicalize().unwrap_or_else(|err| {
                    error!("Can't canonicalize to '{}': {}.", dotfiles_path.display(), err)
                });
                let home_path = Path::new(&home).join(&semi_path);
                // let home_path = home_path.canonicalize().unwrap_or_else(|err| {
                //     error!("Can't canonicalize to '{}': {}.", home_path.display(), err)
                // });

                // Let's naively skip what we've already linked
                if home_path.exists() {
                    continue;
                }
                match file {
                    tsml::FileTree::Regular { .. } => {
                        symlink(&dotfiles_path, &home_path).unwrap_or_else(|err| {
                            error!(
                                "Error while trying to make link for regular file '{}' -> '{}': {}",
                                dotfiles_path.display(),
                                home_path.display(),
                                err
                            )
                        });
                    },
                    tsml::FileTree::Directory { children, .. } => {
                        // Verify if should link or create directory
                        // If there's no children, link, else, create directory
                        if children.is_empty() {
                            symlink(&dotfiles_path, &home_path).unwrap_or_else(|err| {
                                error!(
                                "Error while trying to make link for directory '{}' -> '{}': {}",
                                dotfiles_path.display(),
                                home_path.display(),
                                err
                            );
                            });
                        } else {
                            fs::create_dir_all(&home_path)
                                .expect("Error while trying to create directory.");
                        }
                    },
                    tsml::FileTree::Symlink { .. } => todo!(),
                }
            }
        }
    }
}
