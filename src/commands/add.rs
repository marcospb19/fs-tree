use std::{
    fs,
    io::{BufWriter, Write},
};

use crate::error;

pub fn run_add_command(group_names: &[&str]) {
    let content = fs::read_to_string("dotao.tsml").unwrap();
    let mut tree = tsml::Groups::from_text(&content).unwrap();
    // The header of the file is made of the starting comments and blank lines

    let mut header = tree.info.file_header.clone();

    let amount_of_trailing_empty = header.lines().rev().take_while(|line| line.is_empty()).count();
    // Remove excessive empty lines
    for _ in 0..amount_of_trailing_empty {
        header.pop();
    }

    let group_files: Vec<Vec<tsml::FileTree>> = group_names
        .iter()
        .map(|path| {
            tsml::FileTree::collect_from_directory(path).unwrap_or_else(|err| {
                error!("Error while trying to read `add` arguments: {:?}.", err)
            })
        })
        .collect();

    // Updating groups
    for (name, vec_of_files) in group_names.iter().zip(group_files) {
        *tree.map.entry(name.to_string()).or_default() = vec_of_files;
    }

    // Override file
    let file = fs::File::create("dotao.tsml").unwrap_or_else(|err| {
        error!("Unable to open dotao.tsml to edit (write) it: {}.", err);
    });

    let mut writer = BufWriter::new(file);
    for comment in header.lines() {
        writeln!(writer, "{}", comment).unwrap_or_else(|err| error!("Unable to write! {}", err));
    }
    let tree_content = tsml::groups_to_tsml(&tree);
    write!(writer, "{}", tree_content).unwrap_or_else(|err| error!("Unable to write! {}", err));
}
