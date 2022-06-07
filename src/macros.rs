/// Internal macro used by `dir!`, don't use this one
#[macro_export]
macro_rules! dir_inner {
    // :literal captures "path", the path suffix to each file
    // So create a file with this name and return it
    ($path:literal) => {{
        let path: &::std::path::Path = $path.as_ref();
        $crate::FileTree::new_regular(path)
    }};

    // :expr also captures "path", but we added the :literal arm before, so it's not going to happen
    //
    // If we have not found a literal in the form of "path", try to insert it as a ready-to-use file
    //
    // Example for this arm:
    //   `dir!("nome", [..., dir!("outro_dir", [...]), ... ])`
    //
    // The second `dir!` in the example is a valid File, so just insert it.
    ($file:expr) => {{
        $file
    }};
}

/// Easy way to create directory
///
/// Example:
/// ```
/// # use crate::fs_tree::*;
/// // let file: File<()> = dir!("root", [
/// //     "file1",
/// //     "file2",
/// //     dir!("inner_dir", [
/// //         "more_file1",
/// //         "more_file2"
/// //     ]),
/// //     "file3"
/// // ]);
/// ```
#[macro_export]
macro_rules! dir {
    ($path:expr, [$($args:expr),*] ) => {{
        let mut children = vec![];
        $(
            let file = $crate::dir_inner!( $args );
            children.push(file);
        )*
        let path: &::std::path::Path = $path.as_ref();
        $crate::FileTree::new_directory(path, children)
    }};
}

/// Easy way to create directory
///
/// Example:
/// ```
/// # use crate::fs_tree::*;
/// // let file: File<()> = dir!("root", [
/// //     "file1",
/// //     "file2",
/// //     dir!("inner_dir", [
/// //         "more_file1",
/// //         "more_file2"
/// //     ]),
/// //     "file3"
/// // ]);
/// ```

#[macro_export]
macro_rules! tree {
    ($($any:tt)*) => {{
        let mut result = dir!( $($any)* );
        result.fix();
        result
    }};
}

/// Crate a `fs_tree::File`, can be combined with `dir!`
#[macro_export]
macro_rules! file {
    ($path:expr) => {{
        let path: &::std::path::Path = $path.as_ref();
        $crate::FileTree::new_regular(path)
    }};
}

#[macro_export]
macro_rules! symlink {
    ($source:expr, $target:expr) => {{
        let source: &::std::path::Path = $source.as_ref();
        let target: &::std::path::Path = $target.as_ref();
        $crate::FileTree::new_symlink(source, target)
    }};
}

#[cfg(test)]
mod tests {
    use crate::FileTree;

    #[test]
    fn testing_macros() {
        // #[rustfmt::skip]
        let file: FileTree<()> = tree!("root", [
            "file1",
            file!("file2"),
            dir!("inner_dir", ["more_file1", "more_file2", symlink!("from", "to")]),
            "file3"
        ]);

        // // We might want this one
        // #[rustfmt::skip]
        // let file: File<()> = fs_tree!("root", {
        //     "file1",
        //     "file2",
        //     "inner_dir" => [
        //         "more_file1",
        //         "more_file2"
        //     ],
        //     "file3"
        // });

        #[rustfmt::skip]
        let expected = FileTree::<()>::new_directory("root", vec![
            FileTree::new_regular("root/file1"),
            FileTree::new_regular("root/file2"),
            FileTree::new_directory("root/inner_dir", vec![
                FileTree::new_regular("root/inner_dir/more_file1"),
                FileTree::new_regular("root/inner_dir/more_file2"),
                FileTree::new_symlink("root/inner_dir/from", "root/inner_dir/to"),
            ]),
            FileTree::new_regular("root/file3"),
        ]);

        assert_eq!(file, expected);
    }
}
