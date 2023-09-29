use fs_tree::FsTree;

fn main() {
    let tree = FsTree::from_path("src/").unwrap();

    let iter = tree.paths();

    for path in iter {
        println!("{path:?}");
    }
}
