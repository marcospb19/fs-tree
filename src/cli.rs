use clap::{crate_name, crate_version, App, AppSettings, Arg, SubCommand};

pub fn parse_args() -> clap::ArgMatches<'static> {
    App::new(crate_name!())
        .settings(&[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])
        .version(crate_version!())
        .help_message("Display help information.")
        .version_message("Display version information.")
        .subcommand(
            SubCommand::with_name("add")
                .settings(&[AppSettings::ColoredHelp])
                .arg(
                    Arg::with_name("groups")
                        .required(true)
                        .multiple(true)
                        .help("Groups directories."),
                )
                .aliases(&["a", "insert"])
                .about("Add groups to the tree file."),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .settings(&[AppSettings::ColoredHelp])
                .arg(
                    Arg::with_name("groups")
                        .required(true)
                        .multiple(true)
                        .help("Groups directories."),
                )
                .aliases(&["r", "rm", "delete"])
                .about("Remove groups from the tree file."),
        )
        .subcommand(
            SubCommand::with_name("link")
                .settings(&[AppSettings::ColoredHelp])
                .alias("l")
                .about("Link groups in the tree file."),
        )
        .subcommand(
            SubCommand::with_name("unlink")
                .settings(&[AppSettings::ColoredHelp])
                .alias("u")
                .about("Unlink groups in the tree file."),
        )
        .subcommand(
            SubCommand::with_name("init")
                .settings(&[AppSettings::ColoredHelp])
                .arg(
                    Arg::with_name("force")
                        .long("force")
                        .short("f")
                        .help("Create even if there's no git repository"),
                )
                .alias("i")
                .about("Create tree file."),
        )
        .subcommand(
            SubCommand::with_name("status")
                .settings(&[AppSettings::ColoredHelp])
                .about("Show status of the dotfiles."),
        )
        .get_matches()
}
