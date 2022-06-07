use fs_tree::{FileTree, FtResult};

#[allow(unused_variables)]
fn main() -> FtResult<()> {
    let examples_folder = FileTree::<()>::from_path("examples/")?;

    // Recursive iterator that starts at file `examples_folder`
    // See documentation to see how to apply filters to this FilesIter
    for file in examples_folder.files() {
        // println!("{:#?}", file);
    }

    // Same, but using PathsIter
    for path in examples_folder.paths() {
        println!("{:?}", path);
    }

    // If you want to see each child file
    if let Some(children) = examples_folder.children() {
        for child in children {
            // println!("{:?}", child.path);
        }
    }

    // Alternate way
    if let Some(children) = examples_folder.children() {
        for child in children {
            // println!("{:?}", child.path);
        }
    }

    Ok(())
}
