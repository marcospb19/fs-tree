mod cli;

use dotao::{dotfiles::DotfileGroup, error::*};

use std::process;

fn main() {
    let args = cli::parse_args();

    let mut groups: Vec<DotfileGroup> = vec![];
    let mut error_occurred = false;

    // For each arg of GROUPS
    for group_path in args.values_of("GROUPS").unwrap() {
        // Try to transform into DotfileGroup
        let group: Result<DotfileGroup> = DotfileGroup::from_directory_path(group_path, false);

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
    println!("{:#?}", groups);
}
