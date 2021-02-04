use dotao::{
    dotfiles::DotfileGroup,
    error::*,
    link::{LinkBehavior, LinkInformation},
};

use std::{env, fs, path::PathBuf, process};

fn run_status_command() {
    println!("run status command");
}

fn run_init_command() {
    println!("run init command");
}

fn run_add_command(group_names: &[&str]) {
    let tree: tsml::Groups = gather_tsml_thing();
    let group_files: Vec<tsml::File> = group_names
        .iter()
        .map(|x| {
            tsml::File::from_path(x, false).unwrap_or_else(|err| {
                eprintln!("Unable to add '{}' to the file: '{}'", x, err);
                process::exit(1);
            })
        })
        .collect();

    for (group_file, group_name) in group_files.iter().zip(group_names) {
        if let Some(value) = tree.map.get(*group_name) {
            if value == group_file {
                unimplemented!();
            }
            println!("in: {:?}", group_name);
        } else {
            println!("out: {:?}", group_name);
        }
    }

    println!("run add command");
}

fn run_remove_command() {
    println!("run remove command");
}

fn gather_tsml_thing() -> tsml::Groups {
    tsml::Groups::from_path("dotao.tsml").unwrap_or_else(|err| {
        eprintln!("error trying to open 'dotao.tsml': '{}'", err);
        process::exit(1);
    })
}

pub fn run() {
    // lalalalalalala
    let test_path = "/home/marcospb19/dotfiles";
    std::env::set_current_dir(test_path).unwrap();

    if env::args().len() == 1 {
        run_status_command();
    }
    let args = super::cli::parse_args();

    match args.subcommand() {
        ("status", Some(_)) => run_status_command(),
        ("init", Some(_)) => run_init_command(),
        ("add", Some(add_matches)) => {
            let groups = add_matches.values_of("groups").unwrap(); // Safe
            let groups: Vec<&str> = groups.collect();
            run_add_command(&groups);
        },
        ("rm", Some(_)) => run_remove_command(),
        _ => unreachable!(),
    }

    // let mut groups: Vec<DotfileGroup> = vec![];
    // let mut error_occurred = false;

    // // For each arg of GROUPS
    // for group_path in args.values_of("GROUPS").unwrap() {
    //     // Try to transform into DotfileGroup
    //     // Symlinks in dotfiles work, so follow them
    //     let group: Result<DotfileGroup> =
    // DotfileGroup::from_directory_path(&group_path, true);

    //     if let Ok(group) = group {
    //         groups.push(group);
    //     } else if let Err(err) = group {
    //         error_occurred = true;
    //         // Display customized error message
    //         match err {
    //             DotaoError::ReadError { path, source } => {
    //                 eprintln!(
    //                     "Error: Read error for group '{}': {}: '{}'.",
    //                     group_path,
    //                     source,
    //                     path.display()
    //                 );
    //             },
    //             other_err => eprintln!("Error: {}: '{}'", other_err,
    // group_path),         }
    //     }
    // }

    // if error_occurred {
    //     process::exit(1);
    // }

    // let home_path = env::var("HOME").unwrap_or_else(|err| {
    //     eprintln!("Unable to read env variable HOME: {}", err);
    //     process::exit(1);
    // });
    // let home_path = PathBuf::from(home_path);

    // let interactive_mode = args.is_present("interactive_mode");
    // let overwrite_file = args.is_present("overwrite_file");
    // let overwrite_directory = args.is_present("overwrite_directory");
    // let overwrite_symlink = args.is_present("overwrite_symlink");

    // let link_behavior = LinkBehavior::new(
    //     interactive_mode,
    //     overwrite_file,
    //     overwrite_directory,
    //     overwrite_symlink,
    // );

    // let fake_run = args.is_present("fake-run");
    // if fake_run {
    //     println!("Fake run activated, no changes will be made.");
    // }

    // // println!("{:#?}", link_behavior);

    // let mut link_information = LinkInformation::new();
    // link_information.configure_behavior(link_behavior);
    // for group in groups {
    //     link_information.add_group(group);
    // }
    // link_information
    //     .prepare_linkage_to_home(&home_path)
    //     .unwrap_or_else(|err| {
    //         eprintln!("prepare_linkage_to_home error: {}", err);
    //         process::exit(1);
    //     });

    // if link_information.critical_error_occurred() {
    //     link_information.show_errors();
    //     process::exit(1);
    // }

    // if fake_run {
    //     println!("Skiping link_information.proceed_and_link().");
    // } else {
    //     link_information.proceed_and_link().unwrap_or_else(|err| {
    //         eprintln!("Mds ocorreu um erro!!!!!!!!!!!!!!!!!!!!!!!");
    //         eprintln!("{}", err);
    //     });
    // }

    // println!("{:#?}", link_information.link_behavior);
    // println!("{:#?}", link_information.payload);
}
