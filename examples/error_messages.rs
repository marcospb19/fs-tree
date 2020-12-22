use file_structure::{util::*, FSResult, File};

fn main() {
    let path = "src/ex.rs";

    let temp: FSResult<Vec<File<()>>> = collect_directory_children(path, false);

    if let Err(err) = temp {
        eprintln!("{}", err);
    };
    // if let Err(err) = symlink_target(path) {
    //     eprintln!("{}", err);
    // }
}
