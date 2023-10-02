use fs_tree::FsTree;

fn main() {
    let tree = FsTree::read_at("src/").unwrap();

    let iter = tree.paths();

    for path in iter {
        println!("{path:?}");
    }
}
