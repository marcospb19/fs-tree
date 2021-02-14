use std::{
    env,
    ffi::OsStr,
    fs,
    io::prelude::*,
    path::{Path, PathBuf},
    process,
};

use indoc::indoc;

use super::{cli, error};

fn get_current_dir() -> PathBuf {
    env::current_dir()
        .unwrap_or_else(|err| error!("Failed to read curent directory path: '{}'.", err))
}

fn bytes_to_uft(asd: impl AsRef<OsStr>) -> String {
    let text = format!("{:?}", asd.as_ref());
    text.trim_matches('"').to_string()
}

fn is_currently_in_git_repository() -> bool {
    let current_dir = get_current_dir();
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
    // Checks
    if !is_currently_in_git_repository() && !force_flag {
        error!(
            "You are not inside a git repository, we recommend you to first run `git init`.\n\
            To ignore this recommendation, type `dotao init --force` instead."
        );
    } else if Path::new("dotao.tsml").exists() {
        error!(
            "You passed the --init flag, but there's already a 'dotao.tsml' file in here!\n\
            Run `rm dotao.tsml` before if you wish to restart everything.\n\
            Be careful,"
        );
    }

    let mut new_dotao_tsml = fs::File::create("dotao.tsml")
        .unwrap_or_else(|err| error!("Error while trying to create file 'dotao.tsml': {}.", err));

    write!(
        new_dotao_tsml,
        indoc!(
            "//      __     __              __
             //  ___/ /__  / /____ ____    / /________ ___
             // / _  / _ \\/ __/ _ `/ _ \\  / __/ __/ -_) -_)
             // \\___/\\___/\\__/\\___/\\___/  \\__/_/  \\__/\\__/
             //
             // Tree configuration file, see more at
             // https://github.com/marcospb19/dotao (TODO, there's no info there)
             //
             // Tips: Some commands you can type, and what they do:
             //   - `dotao add <folders>`, to add a group tree to this file.
             //   - `dotao status`, to see what's going on.
             //   - `dotao link`, to link added groups your home directory.
             //
            "
        )
    )
    .unwrap_or_else(|err| error!("Error while trying to write to 'dotao.tsml': {}.", err));

    // Success!
    println!(
        "Tree file successfully created at '{}'.",
        bytes_to_uft(get_current_dir().join("dotao.tsml"))
    );
    println!(
        "For help, type `dotao --help`.\n\
         See also the (TODO) full tutorial at https://github.com/marcospb19/dotao ."
    );
}

fn run_add_command(group_names: &[&str], init_flag: bool, force_flag: bool) {
    if init_flag {
        println!("Running `dotao init` before `dotao add`.");
        run_init_command(force_flag);
    }
    //
    let content = fs::read_to_string("dotao.tsml").unwrap();
    let mut tree = tsml::Groups::from_text(&content);
    let _comment_lines =
        content.lines().take_while(|line| line.starts_with("//")).collect::<Vec<&str>>();

    let group_files: Vec<Vec<tsml::FileTree>> = group_names
        .iter()
        .map(|path| {
            tsml::FileTree::collect_from_directory(path).unwrap_or_else(|err| {
                error!("Error while trying to read `add` arguments: {:?}.", err)
            })
        })
        .collect();

    // Lmao?
    for (name, vec_of_files) in group_names.iter().zip(group_files) {
        *tree.map.entry(name.to_string()).or_default() = vec_of_files;
    }

    println!("--\n{}\n--", tsml::groups_to_tsml(&tree));

    // for group in group_files.iter() {
    //     group.paths().for_each(|x| println!("{:?}", x));
    //     // let mut set = BTreeSet::new();

    //     // vecs.push
    // }

    // for (group_file, group_name) in group_files.iter().zip(group_names) {
    //     if let Some(value) = tree.map.get(*group_name) {
    //         // if value == group_file {
    //         //     unimplemented!();
    //         // }
    //         println!("in: {:?}", group_name);
    //     } else {
    //         println!("out: {:?}", group_name);
    //     }
    // }

    // println!("run add command");
}

fn run_remove_command() {
    println!("run remove command");
}

pub fn run() {
    // temporary fix for fast testing
    let test_path = "/home/marcospb19/dotfiles";
    env::set_current_dir(test_path).expect("Expected, just for testing");

    if env::args().len() == 1 {
        run_status_command();
    }
    let args = cli::parse_args();

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
    //     eprintln!("Unable to read env variable HOME: {}.", err);
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
    //         eprintln!("prepare_linkage_to_home error: {}.", err);
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
    //         eprintln!("{}.", err);
    //     });
    // }

    // println!("{:#?}", link_information.link_behavior);
    // println!("{:#?}", link_information.payload);
}
