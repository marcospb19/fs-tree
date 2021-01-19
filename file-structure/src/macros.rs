/// Internal macro used by `dir!`, don't use this one
#[macro_export]
macro_rules! dir_inner {
    // :literal captures "path", the path suffix to each file
    // So create a file with this name and return it
    ($path:literal) => {{
        let path: &::std::path::Path = $path.as_ref();
        $crate::File::new(path, $crate::FileType::Regular)
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
/// # use file_structure::*;
/// let file: File<()> = dir!("root", [
///     "file1",
///     "file2",
///     dir!("inner_dir", [
///         "more_file1",
///         "more_file2"
///     ]),
///     "file3"
/// ]);
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
        $crate::File::<()>::new(path, $crate::FileType::Directory(children))
    }};
}

/// Crate a `file_structure::File`, can be combined with `dir!`
#[macro_export]
macro_rules! file {
    ($path:expr) => {{
        let path: &::std::path::Path = $path.as_ref();
        $crate::File::new(path, $crate::FileType::Regular)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{File, FileType};

    #[test]
    fn testing_macros() {
        #[rustfmt::skip]
        let file: File<()> = dir!("root", [
            "file1",
            "file2",
            dir!("inner_dir", [
                "more_file1",
                "more_file2"
            ]),
            "file3"
        ]);

        // // We might want this one
        // #[rustfmt::skip]
        // let file: File<()> = file_tree!("root", {
        //     "file1",
        //     "file2",
        //     "inner_dir" => [
        //         "more_file1",
        //         "more_file2"
        //     ],
        //     "file3"
        // });

        #[rustfmt::skip]
        let expected = File::<()>::new("root", FileType::Directory(vec![
            File::new("file1", FileType::Regular),
            File::new("file2", FileType::Regular),
            File::new("inner_dir", FileType::Directory(vec![
                File::new("more_file1", FileType::Regular),
                File::new("more_file2", FileType::Regular),
            ])),
            File::new("file3", FileType::Regular),
        ]));

        assert_eq!(file, expected);
    }
}
