use tree_file::{self, De, ValueType};

use std::fs;

fn main() {
    let path = "examples/simple.tree";
    let text = fs::read_to_string(path).expect("Should succeed");

    //

    let groups_map = tree_file::GroupsMap::from_text(text);

    // You can assume that the group "main" is always present
    let main_group = groups_map.get("main").unwrap();
}
