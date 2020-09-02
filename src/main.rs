mod cli;

fn main() {
    let args = cli::parse_args();
    println!("{:?}", args.value_of("ARG").unwrap());
}
