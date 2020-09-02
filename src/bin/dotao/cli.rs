use clap::*;

pub(crate) fn parse_args() -> clap::ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .after_help("Repository: https://github.com/marcospb19/dotao")
        .settings(&[AppSettings::ColoredHelp])
        .arg(Arg::with_name("ARG")) // Temporary
        .get_matches()
}
