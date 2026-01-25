//! Macros for declaring a [`FsTree`](crate::FsTree).

/// Macro for declaring a [`FsTree`](crate::FsTree) literal.
///
/// # Syntax:
///
/// - `name` is a regular file.
/// - `name: [ ... ]` is a directory.
/// - `name -> name` is a symlink.
/// - Commas are (unfortunately) not supported.
/// - Use quotes (`"name"`) for spaces, dots, etc.
///
/// # Examples:
///
/// ```
/// use fs_tree::{FsTree, tree, TrieMap};
///
/// let result = tree! {
///     file1
///     outer_dir: [
///         file2
///         inner_dir: [
///             file3
///         ]
///         link1 -> target
///         link2 -> "/home/username/.gitconfig"
///     ]
/// };
///
/// let expected = FsTree::Directory(TrieMap::from([
///     ("file1".into(), FsTree::Regular),
///     ("outer_dir".into(), FsTree::Directory(TrieMap::from([
///         ("file2".into(), FsTree::Regular),
///         ("inner_dir".into(), FsTree::Directory(TrieMap::from([
///             ("file3".into(), FsTree::Regular),
///         ]))),
///         ("link1".into(), FsTree::Symlink("target".into())),
///         ("link2".into(), FsTree::Symlink("/home/username/.gitconfig".into())),
///     ]))),
/// ]));
///
/// assert_eq!(result, expected);
/// ```
#[macro_export]
macro_rules! tree {
    ($($all:tt)+) => {{
        let mut trie = $crate::TrieMap::new();
        // Jumps between tree_internal and inner invocations
        $crate::tree_internal!(trie $($all)*);
        $crate::FsTree::Directory(trie)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! tree_internal {
    // Base case for recursive macro (lookup tt-munching)
    ($parent:ident) => {};
    ($parent:ident $path:ident : $($rest:tt)*) => {
        $crate::tree_internal_dir!($parent { ::std::stringify!($path) } $($rest)*)
    };
    ($parent:ident $path:literal : $($rest:tt)*) => {
        $crate::tree_internal_dir!($parent { $path } $($rest)*)
    };
    ($parent:ident { $path:expr } : $($rest:tt)*) => {
        $crate::tree_internal_dir!($parent { $path } $($rest)*)
    };

    // For symlinks we support the cartesian product: S * S, where S := [ident, literal, expr].
    //
    // So we have a second step parsing which is done at the other macro.
    //
    // For the "FROM -> TO", here we're parsing the FROM while tree_internal_symlink
    // will parse the TO.
    ($parent:ident $path:ident -> $($rest:tt)*) => {
        $crate::tree_internal_symlink!($parent { ::std::stringify!($path) } $($rest)*)
    };
    ($parent:ident $path:literal -> $($rest:tt)*) => {
        $crate::tree_internal_symlink!($parent { $path } $($rest)*)
    };
    ($parent:ident { $path:expr } -> $($rest:tt)*) => {
        $crate::tree_internal_symlink!($parent { $path } $($rest)*)
    };

    ($parent:ident $path:ident $($rest:tt)*) => {
        $crate::tree_internal_regular!($parent { ::std::stringify!($path) } $($rest)*);
    };
    ($parent:ident $path:literal $($rest:tt)*) => {
        $crate::tree_internal_regular!($parent { $path } $($rest)*);
    };
    ($parent:ident { $path:expr } $($rest:tt)*) => {
        $crate::tree_internal_regular!($parent { $path } $($rest)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tree_internal_dir {
    ($parent:ident { $path:expr } [ $($inner:tt)* ] $($rest:tt)*) => {
        #[allow(unused_mut)]
        let mut node = $crate::TrieMap::new();
        $crate::tree_internal!(node $($inner)*);
        $parent.insert(
            ::std::path::PathBuf::from($path),
            $crate::FsTree::Directory(node)
        );
        $crate::tree_internal!($parent $($rest)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tree_internal_regular {
    ($parent:ident { $path:expr } $($rest:tt)*) => {
        $parent.insert(
            ::std::path::PathBuf::from($path),
            $crate::FsTree::Regular
        );
        $crate::tree_internal!($parent $($rest)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tree_internal_symlink {
    // Parse step 2
    ($parent:ident { $path:expr } $target:ident $($rest:tt)*) => {
        $crate::tree_internal_symlink!(@done $parent { $path } { ::std::stringify!($target) } $($rest)*)
    };
    ($parent:ident { $path:expr } $target:literal $($rest:tt)*) => {
        $crate::tree_internal_symlink!(@done $parent { $path } { $target } $($rest)*)
    };
    ($parent:ident { $path:expr } { $target:expr } $($rest:tt)*) => {
        $crate::tree_internal_symlink!(@done $parent { $path } { $target } $($rest)*)
    };

    // All done
    (@done $parent:ident { $path:expr } { $target:expr } $($rest:tt)*) => {
        $parent.insert(
            ::std::path::PathBuf::from($path),
            $crate::FsTree::Symlink(::std::path::PathBuf::from($target))
        );
        $crate::tree_internal!($parent $($rest)*)
    };
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;

    use crate::{FsTree, TrieMap};

    #[test]
    fn test_macro_compiles_with_literals_and_idents() {
        tree! {
            "folder": [
                folder: [
                    file
                    "file"
                    link -> target
                    link -> "target"
                    "link" -> target
                    "link" -> "target"
                ]
            ]
        };
    }

    #[test]
    fn test_tree_macro_single_regular_file() {
        let result = tree! {
            file
        };
        let expected = FsTree::Directory(TrieMap::from([("file".into(), FsTree::Regular)]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tree_macro_empty_directory() {
        let result = tree! { dir: [] };
        let expected = FsTree::Directory(TrieMap::from([("dir".into(), FsTree::new_dir())]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tree_macro_single_symlink() {
        let result = tree! {
            link -> target
        };

        let expected = FsTree::Directory(TrieMap::from([(
            "link".into(),
            FsTree::Symlink("target".into()),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tree_macro_nested_directories() {
        let result = tree! {
            outer_dir: [
                inner_dir: []
            ]
        };

        let expected = {
            let mut tree = FsTree::new_dir();
            tree.insert("outer_dir", FsTree::Directory(TrieMap::new()));
            tree.insert("outer_dir/inner_dir", FsTree::Directory(TrieMap::new()));
            tree
        };

        assert_eq!(result, expected);
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

        let expected = {
            let mut tree = FsTree::new_dir();
            tree.insert("config", FsTree::Regular);
            tree.insert("outer_dir", FsTree::Directory(TrieMap::new()));
            tree.insert("outer_dir/file1", FsTree::Regular);
            tree.insert("outer_dir/file2", FsTree::Regular);
            tree.insert("link", FsTree::Symlink("target".into()));
            tree
        };

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
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

        let expected = FsTree::Directory(TrieMap::from([
            ("config1".into(), FsTree::Regular),
            ("config2".into(), FsTree::Regular),
            ("outer_dir".into(), FsTree::Directory(TrieMap::from([
                ("file1".into(), FsTree::Regular),
                ("file2".into(), FsTree::Regular),
                ("inner_dir".into(), FsTree::Directory(TrieMap::from([
                    ("inner1".into(), FsTree::Regular),
                    ("inner2".into(), FsTree::Regular),
                    ("inner3".into(), FsTree::Regular),
                    ("inner_link".into(), FsTree::Symlink("inner_target".into())),
                ]))),
            ]))),
            ("link".into(), FsTree::Symlink("target".into())),
            ("config3".into(), FsTree::Regular),
        ]));

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_tree_macro_with_expressions() {
        let config = |index: i32| format!("config{index}");

        let result = tree! {
            {config(1)}
            {"config2".to_string()}
            "outer_dir": [
                {{
                    let mut string = String::new();
                    string.push_str("file");
                    string.push_str("1");
                    string
                }}
                file2
                {format!("inner") + "_" + "dir"}: [
                    inner1
                    {{"inner2"}}
                    inner3
                    { format!("inner_link") } -> { ["inner_target"][0] }
                ]
            ]
            link -> { PathBuf::from("target") }
            config3
        };

        let expected = FsTree::Directory(TrieMap::from([
            ("config1".into(), FsTree::Regular),
            ("config2".into(), FsTree::Regular),
            ("outer_dir".into(), FsTree::Directory(TrieMap::from([
                ("file1".into(), FsTree::Regular),
                ("file2".into(), FsTree::Regular),
                ("inner_dir".into(), FsTree::Directory(TrieMap::from([
                    ("inner1".into(), FsTree::Regular),
                    ("inner2".into(), FsTree::Regular),
                    ("inner3".into(), FsTree::Regular),
                    ("inner_link".into(), FsTree::Symlink("inner_target".into())),
                ]))),
            ]))),
            ("link".into(), FsTree::Symlink("target".into())),
            ("config3".into(), FsTree::Regular),
        ]));

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_tree_macro_with_symlinks_all_possibilities() {

        // Cartesian product S * S where S := [ident, literal, expr]
        let result = tree! {
            a1 -> b1
            a2 -> "b2"
            a3 -> {"b3"}
            "a4" -> b4
            "a5" -> "b5"
            "a6" -> {"b6"}
            {"a7"} -> b7
            {"a8"} -> "b8"
            {"a9"} -> {"b9"}
        };

        let expected = FsTree::Directory(TrieMap::from([
            ("a1".into(), FsTree::Symlink("b1".into())),
            ("a2".into(), FsTree::Symlink("b2".into())),
            ("a3".into(), FsTree::Symlink("b3".into())),
            ("a4".into(), FsTree::Symlink("b4".into())),
            ("a5".into(), FsTree::Symlink("b5".into())),
            ("a6".into(), FsTree::Symlink("b6".into())),
            ("a7".into(), FsTree::Symlink("b7".into())),
            ("a8".into(), FsTree::Symlink("b8".into())),
            ("a9".into(), FsTree::Symlink("b9".into())),
        ]));

        assert_eq!(result, expected);
    }
}
