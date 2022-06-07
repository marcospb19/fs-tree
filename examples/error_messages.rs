// This is a test for us to see what are the error messages this gives us.
//
use fs_tree::{util, FileTree};

fn main() {
    let path = "Cargo.toml";

    if let Err(err) = FileTree::collect_from_directory(path) {
        eprintln!("{}", err);
    }
    if let Err(err) = util::symlink_target(path) {
        eprintln!("{}", err);
    }
}
