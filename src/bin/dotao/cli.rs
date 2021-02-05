use clap::{crate_name, crate_version, App, AppSettings, Arg, SubCommand};

pub(super) fn parse_args() -> clap::ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .settings(&[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])
        // Why isn't this working as intended?
        // .about("See --help for more detailed help.")
        // .long_about("See -h for shorter help.")
        .help_message("Display help information.")
        .version_message("Display version information.")
        .subcommand(
            SubCommand::with_name("add")
                .alias("insert")
                .arg(
                    Arg::with_name("groups")
                        .required(true)
                        .multiple(true)
                        .help("pass each group folder"),
                )
                .arg(Arg::with_name("init").long("init").help("Run `dotao init` beforehand."))
                .arg(
                    Arg::with_name("force")
                        .long("force")
                        .short("f")
                        .help("Add even if there's no "),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .arg(
                    Arg::with_name("groups")
                        .required(true)
                        .multiple(true)
                        .help("Group folders to remove from the tree"),
                )
                .alias("rm"),
        )
        .subcommand(
            SubCommand::with_name("init").arg(
                Arg::with_name("force")
                    .long("force")
                    .short("f")
                    .help("Create even if there's no git repository"),
            ),
        )
        // .subcommand(
        //     SubCommand::with_name("add").arg(
        //         Arg::with_name("GROUPS")
        //             .multiple(true)
        //             .help("Groups folders to add"),
        //     ),
        // )
        // .arg(
        //     Arg::with_name("GROUPS")
        //         .multiple(true)
        //         .required(true)
        //         .help("List of dotfile groups that will be linked the HOME directory."),
        // )
        // .arg(
        //     Arg::with_name("interactive_mode")
        //         .short("i")
        //         .help("Run in interactive mode, try to solve conflicts with questions,"),
        // )
        // .arg(
        //     Arg::with_name("overwrite_symlink")
        //         .short("s")
        //         .help("Overwrite symlinks."),
        // )
        // .arg(
        //     Arg::with_name("overwrite_file")
        //         .short("f")
        //         .help("Overwrite files."),
        // )
        // .arg(
        //     Arg::with_name("overwrite_directory")
        //         .short("d")
        //         .help("Overwrite directories."),
        // )
        // .arg(
        //     Arg::with_name("fake-run")
        //         .long("fake")
        //         .help("Don't mess with files."),
        // )
        .get_matches()
}
