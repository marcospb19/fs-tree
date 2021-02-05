use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use dotao::{
    dotfiles::DotfileGroup,
    error::*,
    link::{LinkBehavior, LinkInformation},
};

use super::error;

fn is_currently_in_git_repository() -> bool {
    let current_dir =
        env::current_dir().unwrap_or_else(|err| error! {"Failed to read curent directory path: '{}'.", err});
    let mut path: &Path = &current_dir;
    loop {
        if path.join(".git").exists() {
            return true;
        } else if let Some(parent) = path.parent() {
            path = parent;
        } else {
            return false;
        }
    }
}

fn run_status_command() {
    println!("run status command");
}

fn run_init_command(force_flag: bool) {
    if !is_currently_in_git_repository() && !force_flag || true {
        error! {
            "You are not inside a git repository, we recommend you to first run `git init`.\n\
            To ignore this recommendation, type `dotao init --force` instead."
        };
    }

    if Path::new("dotao.tsml").exists() {
        error!("You passed the --init flag, but there's already a 'dotao.tsml' file in here! Aborting.");
    }
    let fs_file = fs::File::open("dotao.tsml");
    println!("run init command");
}

fn run_add_command(group_names: &[&str], init_flag: bool, force_flag: bool) {
    if init_flag {
        run_init_command(force_flag);
    }
    let tree: tsml::Groups = gather_tsml_thing();
    let group_files: Vec<tsml::File> = group_names
        .iter()
        .map(|x| {
            tsml::File::from_path(x, false)
                .unwrap_or_else(|err| error! {"Unable to add '{}' to the file: '{}'", x, err})
        })
        .collect();

    for (group_file, group_name) in group_files.iter().zip(group_names) {
        if let Some(value) = tree.map.get(*group_name) {
            // if value == group_file {
            //     unimplemented!();
            // }
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
    env::set_current_dir(test_path).expect("Expected, just for testing");

    if env::args().len() == 1 {
        run_status_command();
    }
    let args = super::cli::parse_args();

    match args.subcommand() {
        ("status", Some(_)) => run_status_command(),
        ("init", Some(init_matches)) => {
            // Flag
            let force = init_matches.is_present("force");
            run_init_command(force);
        },
        ("add", Some(add_matches)) => {
            let groups = add_matches.values_of("groups").unwrap(); // Safe
            let groups: Vec<&str> = groups.collect();
            // Flags
            let init = add_matches.is_present("init");
            let force = add_matches.is_present("force");
            run_add_command(&groups, init, force);
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
