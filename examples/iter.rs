use fs_tree::tree;

fn main() {
    let tree = tree! {
        folder: {
            file1
            file2
        }
    };

    // Other available iterators
    let _iter = tree.paths();
    let _iter = tree.nodes();

    for (_node, path) in tree.iter() {
        println!("{path:?}");
    }
}
