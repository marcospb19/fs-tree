use std::{fs, io::Write, path::Path};

use indoc::indoc;

use crate::{
    error,
    util::{bytes_to_uft, is_currently_in_git_repository, is_in_dotfiles_folder, CURRENT_DIR},
};

pub fn run_init_command(force_flag: bool) {
    // Checks
    if !is_currently_in_git_repository() && !force_flag {
        error!(
            "You are not inside a git repository, we recommend you to first run `git init`.\n\
             To ignore this recommendation, run `dotao init --force` instead."
        );
    } else if !is_in_dotfiles_folder() && !force_flag {
        error!(
            "You are not inside the '~/dotfiles' folder, we recomend creating it and running `dotao` in it.\n\
             To ignore this recommendation, run `dotao init --force` instead."
        );
    } else if Path::new("dotao.tsml").exists() {
        error!(
            "You ran `dotao init`, but the 'dotao.tsml' file already exists!\n\
             Delete the file manually if you with to restart the tree configuration.\n\
             This action may not be reversible."
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
             // Tree configuration file, see more at
             // https://github.com/marcospb19/dotao (TODO, there's no info there)
             //
             // Tips: Some commands you can type, and what they do:
             //   - `dotao add <folders>`, to add a group tree to this file.
             //   - `dotao status`, to see what's going on.
             //   - `dotao link`, to link added groups your home directory.
             //
             //
             // (Only the comments in this header block are persistent)
            "
        )
    )
    .unwrap_or_else(|err| error!("Error while trying to write to 'dotao.tsml': {}.", err));

    // Success!
    println!(
        "Tree file successfully created at '{}'.",
        bytes_to_uft(CURRENT_DIR.join("dotao.tsml"))
    );
    println!(
        "For help, type `dotao --help`.\n\
         See also the (TODO) full tutorial at https://github.com/marcospb19/dotao ."
    );
}
