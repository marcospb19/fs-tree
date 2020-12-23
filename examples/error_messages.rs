use file_structure::util::*;

fn main() {
    let path = "src/ex.rs";

    if let Err(err) = collect_directory_children::<(), &str>(path, false) {
        eprintln!("{}", err);
    }
    if let Err(err) = symlink_target(path) {
        eprintln!("{}", err);
    }
}
