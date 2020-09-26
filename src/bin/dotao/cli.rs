use clap::*;
use AppSettings::*;

pub(super) fn parse_args() -> clap::ArgMatches<'static> {
    let mut version = String::from(crate_version!());
    version.push_str("\nhttps://github.com/marcospb19/dotao");

    App::new(crate_name!())
        .settings(&[ColoredHelp, ArgRequiredElseHelp])
        .version(version.as_ref())
        .about("See --help for more detailed help.")
        .long_about("See -h for shorter help.")
        .help_message("Display help information.")
        .version_message("Display version information.")
        .arg(
            Arg::with_name("GROUPS")
                .multiple(true)
                .required(true)
                .help("List of dotfile groups that will be linked the HOME directory."),
        )
        .arg(
            Arg::with_name("interactive_mode")
                .short("i")
                .help("Run in interactive mode, try to solve conflicts with questions,"),
        )
        .arg(
            Arg::with_name("overwrite_symlink")
                .short("s")
                .help("Overwrite symlinks."),
        )
        // .arg(
        //     Arg::with_name("dont_overwrite_symlink")
        //         .short("S")
        //         .help("Don't overwrite symlinks."),
        // )
        .arg(
            Arg::with_name("overwrite_file")
                .short("f")
                .help("Overwrite files."),
        )
        // .arg(
        //     Arg::with_name("dont_overwrite_file")
        //         .short("F")
        //         .help("Don't overwrite files."),
        // )
        .arg(
            Arg::with_name("overwrite_directory")
                .short("d")
                .help("Overwrite directories."),
        )
        // .arg(
        //     Arg::with_name("dont_overwrite_directory")
        //         .short("D")
        //         .help("Don't overwrite directories."),
        // )
        .arg(
            Arg::with_name("fake-run")
                .long("fake")
                // .aliases(&["fake", "dry-run", "dry"])
                .help("Don't mess with files."),
        )
        .get_matches()
}
