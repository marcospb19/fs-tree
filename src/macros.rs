#[macro_export]
macro_rules! tree {
    ($($all:tt)*) => {{
        let mut vec = std::vec::Vec::<$crate::FsTree>::new();
        $crate::tree_internal!(vec $($all)*);
        vec
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! tree_internal {
    // Base case
    ($vec:ident) => {};
    // Directory
    ($vec:ident $path:ident : [ $($inner:tt)* ] $($rest:tt)*) => {
        #[allow(unused_mut)]
        let mut inner_dir = std::vec::Vec::<$crate::FsTree>::new();
        $crate::tree_internal!(inner_dir $($inner)*);
        $vec.push($crate::FsTree::new_directory(stringify!($path), inner_dir));
        $crate::tree_internal!($vec $($rest)*)
    };
    // Symlink
    ($vec:ident $path:ident -> $target:ident $($rest:tt)*) => {
        $vec.push($crate::FsTree::new_symlink(stringify!($path), stringify!($target)));
        $crate::tree_internal!($vec $($rest)*)
    };
    // Regular file
    ($vec:ident $path:ident $($rest:tt)*) => {
        $vec.push($crate::FsTree::new_regular(stringify!($path)));
        $crate::tree_internal!($vec $($rest)*);
    };
}

#[cfg(test)]
mod tests {
    use crate::FsTree;

    #[test]
    fn test_tree_macro_single_regular_file() {
        let result = tree! {
            file
        };

        let expected = FsTree::new_regular("file");

        assert_eq!(result, vec![expected]);
    }

    #[test]
    fn test_tree_macro_empty_directory() {
        let result = tree! {
            dir: []
        };

        let expected = FsTree::new_directory("dir", vec![]);

        assert_eq!(result, vec![expected]);
    }

    #[test]
    fn test_tree_macro_single_symlink() {
        let result = tree! {
            link -> target
        };

        let expected = FsTree::new_symlink("link", "target");

        assert_eq!(result, vec![expected]);
    }

    #[test]
    fn test_tree_macro_nested_directories() {
        let result = tree! {
            outer_dir: [
                inner_dir: [
                ]
            ]
        };

        let expected = FsTree::new_directory(
            "outer_dir",
            vec![FsTree::new_directory("inner_dir", vec![])],
        );

        assert_eq!(result, vec![expected]);
    }

    #[test]
    fn test_tree_macro_mixed_types() {
        let result = tree! {
            config
            outer_dir: [
                file1
                file2
            ]
            link -> target
        };

        let expected = vec![
            FsTree::new_regular("config"),
            FsTree::new_directory(
                "outer_dir",
                vec![FsTree::new_regular("file1"), FsTree::new_regular("file2")],
            ),
            FsTree::new_symlink("link", "target"),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tree_macro_big_example() {
        let result = tree! {
            config1
            config2
            outer_dir: [
                file1
                file2
                inner_dir: [
                    inner1
                    inner2
                    inner3
                    inner_link -> inner_target
                ]
            ]
            link -> target
            config3
        };

        let expected = vec![
            FsTree::new_regular("config1"),
            FsTree::new_regular("config2"),
            FsTree::new_directory(
                "outer_dir",
                vec![
                    FsTree::new_regular("file1"),
                    FsTree::new_regular("file2"),
                    FsTree::new_directory(
                        "inner_dir",
                        vec![
                            FsTree::new_regular("inner1"),
                            FsTree::new_regular("inner2"),
                            FsTree::new_regular("inner3"),
                            FsTree::new_symlink("inner_link", "inner_target"),
                        ],
                    ),
                ],
            ),
            FsTree::new_symlink("link", "target"),
            FsTree::new_regular("config3"),
        ];

        assert_eq!(result, expected);
    }
}
