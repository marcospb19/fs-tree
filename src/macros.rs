//! Macros for declaring a [`FsTree`](crate::FsTree).

/// Macro for declaring a [`FsTree`](crate::FsTree) literal.
///
/// # Syntax:
///
/// - `name` is a regular file.
/// - `name: { ... }` is a directory.
/// - `name -> name` is a symlink.
/// - Commas are optional, you can separate items with newlines or whitespace.
/// - Use quotes (`"name"`) for spaces, dots, etc.
///
/// # Examples:
///
/// ```
/// use fs_tree::{FsTree, tree, TrieMap};
///
/// let result = tree! {
///     file1
///     outer_dir: {
///         file2
///         inner_dir: {
///             file3
///         }
///         link1 -> target
///         link2 -> "/home/username/.gitconfig"
///     }
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
        $crate::trees_internal!(trie $($all)*);
        $crate::FsTree::Directory(trie)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! trees_internal {
    // Base case
    ($parent_trie:ident $(,)?) => {};
    // Directory
    ($parent_trie:ident $path:ident : { $($inner:tt)* } $(,)? $($rest:tt)*) => {
        #[allow(unused_mut)]
        let mut trie = $crate::TrieMap::new();
        $crate::trees_internal!(trie $($inner)*);
        $parent_trie.insert(
            ::std::path::PathBuf::from(stringify!($path)),
            $crate::FsTree::Directory(trie)
        );
        $crate::trees_internal!($parent_trie $($rest)*)
    };
    // Directory variation
    ($parent_trie:ident $path:literal : { $($inner:tt)* } $(,)? $($rest:tt)*) => {
        #[allow(unused_mut)]
        let mut trie = $crate::TrieMap::new();
        $crate::trees_internal!(trie $($inner)*);
        $parent_trie.insert(
            ::std::path::PathBuf::from($path),
            $crate::FsTree::Directory(trie)
        );
        $crate::trees_internal!($parent_trie $($rest)*)
    };
    // Symlink
    ($parent_trie:ident $path:ident -> $target:ident $(,)? $($rest:tt)*) => {
        $parent_trie.insert(
            ::std::path::PathBuf::from(stringify!($path)),
            $crate::FsTree::Symlink(::std::path::PathBuf::from(stringify!($target)))
        );
        $crate::trees_internal!($parent_trie $($rest)*)
    };
    // Symlink variation
    ($parent_trie:ident $path:literal -> $target:ident $(,)? $($rest:tt)*) => {
        $parent_trie.insert(
            ::std::path::PathBuf::from($path),
            $crate::FsTree::Symlink(::std::path::PathBuf::from(stringify!($target)))
        );
        $crate::trees_internal!($parent_trie $($rest)*)
    };
    // Symlink variation
    ($parent_trie:ident $path:ident -> $target:literal $(,)? $($rest:tt)*) => {
        $parent_trie.insert(
            ::std::path::PathBuf::from(stringify!($path)),
            $crate::FsTree::Symlink(::std::path::PathBuf::from($target))
        );
        $crate::trees_internal!($parent_trie $($rest)*)
    };
    // Symlink variation
    ($parent_trie:ident $path:literal -> $target:literal $(,)? $($rest:tt)*) => {
        $parent_trie.insert(
            ::std::path::PathBuf::from($path),
            $crate::FsTree::Symlink(::std::path::PathBuf::from($target))
        );
        $crate::trees_internal!($parent_trie $($rest)*)
    };
    // Regular file
    ($parent_trie:ident $path:ident $(,)? $($rest:tt)*) => {
        $parent_trie.insert(
            ::std::path::PathBuf::from(stringify!($path)),
            $crate::FsTree::Regular
        );
        $crate::trees_internal!($parent_trie $($rest)*);
    };
    // Regular file
    ($parent_trie:ident $path:literal $(,)? $($rest:tt)*) => {
        $parent_trie.insert(
            ::std::path::PathBuf::from($path),
            $crate::FsTree::Regular
        );
        $crate::trees_internal!($parent_trie $($rest)*);
    };
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{FsTree, TrieMap};

    #[test]
    fn test_macro_compiles_with_literals_and_idents() {
        tree! {
            "folder": {
                folder: {
                    file
                    "file"
                    link -> target
                    link -> "target"
                    "link" -> target
                    "link" -> "target"
                }
            }
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
        let result = tree! {
            dir: {}
        };

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
            outer_dir: {
                inner_dir: {
                }
            }
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
            outer_dir: {
                file1
                file2
            }
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
            outer_dir: {
                file1
                file2
                inner_dir: {
                    inner1
                    inner2
                    inner3
                    inner_link -> inner_target
                }
            }
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

    #[test]
    fn test_tree_macro_big_example_with_commas() {
        let result = tree! {
            config1,
            config2,
            outer_dir: {
                file1,
                file2,
                inner_dir: {
                    inner1,
                    inner2,
                    inner3,
                    inner_link -> inner_target,
                },
            },
            link -> target,
            config3,
        };

        let expected = tree! {
            config1
            config2
            outer_dir: {
                file1
                file2
                inner_dir: {
                    inner1
                    inner2
                    inner3
                    inner_link -> inner_target
                }
            }
            link -> target
            config3
        };

        assert_eq!(result, expected);
    }
}
