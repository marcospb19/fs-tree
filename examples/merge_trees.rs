use fs_tree::{tree, FsTree};

fn main() {
    let a = tree! {
        unique_a
        shared_dir: {
            inner_shared_file
            inner_unique_a
        }
    };

    let b = tree! {
        unique_b
        shared_dir: {
            inner_shared_file
            inner_unique_b
        }
    };

    let merged = a.try_merge(b);

    let expected = tree! {
        unique_a
        unique_b
        shared_dir: {
            inner_shared_file
            inner_unique_a
            inner_unique_b
        }
    };

    assert_eq!(Some(expected), merged);
}
