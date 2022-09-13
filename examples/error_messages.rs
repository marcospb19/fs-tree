// This is a test for us to see what are the error messages this gives us.
//
use fs_tree::{util, FsTree};

fn main() {
    let path = "Cargo.toml";

    if let Err(err) = FsTree::collect_from_directory(path) {
        eprintln!("{}", err);
    }
    if let Err(err) = util::symlink_follow(path) {
        eprintln!("{}", err);
    }
}
