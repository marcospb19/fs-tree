/// Wraps `clap` CLI argparsing configuration.
mod cli;

use dotao::{
    dotfiles::DotfileGroup,
    error::*,
    link::{Link, LinkBehavior},
};

use std::{env, path::PathBuf, process};

fn main() {
    std::env::set_current_dir("/home/marcospb19/dotfiles").unwrap();
    let args = cli::parse_args();

    let mut groups: Vec<DotfileGroup> = vec![];
    let mut error_occurred = false;

    // For each arg of GROUPS
    for group_path in args.values_of("GROUPS").unwrap() {
        // Try to transform into DotfileGroup
        // Symlinks in dotfiles work, so follow them
        let group: Result<DotfileGroup> = DotfileGroup::from_directory_path(group_path, true);

        if let Ok(group) = group {
            groups.push(group);
        } else if let Err(err) = group {
            error_occurred = true;
            // Display customized error message
            match err {
                DotaoError::ReadError { path, source } => {
                    eprintln!(
                        "Error: Read error for group '{}': {}: '{}'.",
                        group_path,
                        source,
                        path.display()
                    );
                },
                other_err => eprintln!("Error: {}: '{}'", other_err, group_path),
            }
        }
    }

    if error_occurred {
        process::exit(1);
    }

    let home_path = env::var("HOME").unwrap_or_else(|err| {
        eprintln!("Unable to read env variable HOME: {}", err);
        process::exit(1);
    });
    let home_path = PathBuf::from(home_path);

    let link_behavior = if args.is_present("overwrite") {
        LinkBehavior::new(true, true)
    } else {
        Default::default()
    };

    for group in groups {
        group.link_to_home(&home_path, &link_behavior).unwrap();
    }
}
