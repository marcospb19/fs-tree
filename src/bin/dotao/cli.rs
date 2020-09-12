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
            Arg::with_name("overwrite")
                .long("overwrite")
                .short("O")
                .help("Overwrite files, directories and symlinks."),
        )
        .get_matches()
}
