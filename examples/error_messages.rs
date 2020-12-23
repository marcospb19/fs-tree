use file_structure::{util::*, File, FsResult};

fn main() {
    let path = "src/ex.rs";

    let temp: FsResult<Vec<File<()>>> = collect_directory_children(path, false);

    if let Err(err) = temp {
        eprintln!("{}", err);
    };
    // if let Err(err) = symlink_target(path) {
    //     eprintln!("{}", err);
    // }
}
