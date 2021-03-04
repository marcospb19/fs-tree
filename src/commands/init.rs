use std::{fs, io::Write, path::Path};

use indoc::indoc;

use crate::{
    error,
    util::{current_dir, is_in_dotfiles_folder, to_utf},
};

pub fn run_init_command(force_flag: bool) {
    // Checks
    if !is_in_dotfiles_folder() && !force_flag {
        error!(
            "You are not inside the '~/dotfiles' folder, we recomend creating it and running `dotao` in it.\n\
             To ignore this recommendation, run `dotao init --force` instead."
        );
    } else if Path::new("dotao.tsml").exists() {
        error!(
            "You ran `dotao init`, but the 'dotao.tsml' file already exists!\n\
             Delete the file manually if you with to restart the tree configuration.\n\
             \n\
             This action might be irreversible."
        );
    }

    let mut new_dotao_tsml = fs::File::create("dotao.tsml")
        .unwrap_or_else(|err| error!("Error while trying to create file 'dotao.tsml': {}.", err));

    write!(
        new_dotao_tsml,
        indoc!(
            "//       __     __              __
             //   ___/ /__  / /____ ____    / /________ ___
             //  / _  / _ \\/ __/ _ `/ _ \\  / __/ __/ -_) -_)
             //  \\___/\\___/\\__/\\___/\\___/  \\__/_/  \\__/\\__/
             //
             // Welcome to the dotao tree!
             //
             // THis file is used by dotao to keep track of dotfiles, when you run `dotao add` or `dotao remove`, files
             // are added and removed from here. If you're using git, add and push this too.
             //
             // If you're lost, check the quickstart guide (TODO) at https://github.com/marcospb19/dotao
             //
             // You can edit this file manually too, but this is not strictly necessary, as `dotao` might have all the commands needed.
             //
             // ---
             // # Syntax
             // Group syntax:
             // - [group_name]
             //
             // File syntax:
             // \"file_name\"
             //
             // Directory syntax:
             // \"directory_name\": [
             //     \"file_a\"
             //     \"nested_directory\": [
             //         \"file_1\"
             //         \"file_2\"
             //         \"file_3\"
             //         \"file_4\"
             //     ]
             //     \"file_b\"
             //     \"empty_directory\": []
             // ]
             //
             // Tag syntax:
             // (tag_name) [Token] // Where token is a file or directory
             // or
             // (tag_name)
             // - [group_name] // When applying to a group
             //
            "
        )
    )
    .unwrap_or_else(|err| error!("Error while trying to write text to 'dotao.tsml': {}.", err));

    // Success!
    println!("Tree file successfully created at '{}'.", to_utf(current_dir().join("dotao.tsml")));
    println!(
        "For help, type `dotao --help`.\n\
         See also the (TODO) full tutorial at https://github.com/marcospb19/dotao ."
    );
}
