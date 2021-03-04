use std::fs;

use tsml::{Groups, TsmlResult};

fn main() -> TsmlResult<()> {
    // let path = "examples/simplest.tree";
    // let path = "examples/simple.tree";
    // let path = "examples/multiple_groups.tree";
    let path = "examples/dotao.tree";
    let text = fs::read_to_string(path)?;

    let groups = Groups::from_text(&text);
    println!("{:#?}", groups);

    Ok(())
}
