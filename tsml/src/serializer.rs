use crate::{FileTree, Flags, Groups};

// TODO: preserve comments, somehow
// TODO: fix order
// TODO: preserve order of values (iter.rs)

// Where T is whatever it wants to be
pub fn groups_to_tsml(groups: &Groups) -> String {
    let mut text = String::new();
    for (i, group) in groups.map.iter().enumerate() {
        // Group separator
        if i != 0 {
            text.push('\n');
        }
        let (key, files) = group;
        add_group_to_tsml(&mut text, key, files);
    }
    text
}

fn indent(text: &mut String, levels: usize) {
    for _ in 0..levels {
        text.push_str("    ");
    }
}

fn add_flags(text: &mut String, flags: &Option<Flags>) {
    if let Some(flags) = flags {
        if !flags.direct_flags.is_empty() {
            // TODO: preserve order of the flags
            text.push_str(format!("({}", flags.direct_flags.iter().next().unwrap()).as_str());
            for flag in flags.direct_flags.iter().skip(1) {
                text.push_str(format!(", {}", flag).as_str());
            }
            text.push_str(") ");
        }
    } // end of adding flags
}

fn close_bracket(text: &mut String, at_indent_level: usize) {
    indent(text, at_indent_level);
    text.push_str("]\n");
}

// Todo: think about what to do here to deal with group flags
fn add_group_to_tsml(text: &mut String, key: &str, files: &[FileTree]) {
    text.push_str(format!("- [{}]\n", key).as_str());

    let mut last_depth = 0;
    for file_tree in files.iter() {
        // For each file inside of this bad boi
        let mut file_iter = file_tree.files();
        while let Some(file) = file_iter.next() {
            // If we exited a directory, add a closing ']'
            if file_iter.depth() < last_depth {
                close_bracket(text, file_iter.depth());
            }

            // Indent
            indent(text, file_iter.depth());
            // Add flags
            add_flags(text, file.extra());
            // Write file name
            // TODO fix string_lossy

            let file: &FileTree = file;

            text.push_str(
                format!(
                    "\"{}\"",
                    file.path().to_str().expect("We are not supporting non utf-8 paths")
                )
                .as_str(),
            );
            // Draw regular file
            match file {
                FileTree::Regular { .. } => {},
                FileTree::Directory { .. } => {
                    text.push_str(": [");
                },
                FileTree::Symlink { target_path, .. } => {
                    text.push_str(&format!(" > \"{}\"", target_path.to_string_lossy()));
                },
            }
            text.push('\n');
            last_depth = file_iter.depth();
        }
    }

    // If there's any depth left, close all brackets
    for level in (0..last_depth).rev() {
        close_bracket(text, level);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::Flags;

//     #[test]
//     fn asd() {
//         let file = File::new("asd", FileType::<Flags>::Regular);
//         for a in file.files() {
//             println!("{:?}", a);
//         }
//         panic!();
//     }
// }
