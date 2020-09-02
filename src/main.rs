mod cli;

use dotao::dotfiles::DotfileGroup;

fn main() {
    let args = cli::parse_args();
    let arg = args.value_of("ARG").unwrap();
    let group = DotfileGroup::new(arg);
    println!("{:#?}", group);
}
