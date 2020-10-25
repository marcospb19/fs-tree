use tree_file::Groups;

use std::{fs, io};

fn main() -> io::Result<()> {
    let path = "examples/simplest.tree";
    let text = fs::read_to_string(path)?;

    let groups = Groups::from_text(&text);
    println!("{:#?}", groups);

    Ok(())
}
