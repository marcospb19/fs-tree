use logos::Logos;
use tree_file::get_groups;
use tree_file::Group;

use std::fs;

// impl std::ops::Index<&str> for Vec<Group> {
//     type Output = usize;

//     fn index(&self, _: Self) -> Self::Output {
//         10
//     }
// }

fn main() {
    let text = fs::read_to_string("examples/dotao.tree").expect("Missing examples/dotao.tree");
    let groups = get_groups(&text);

    for group in &groups {
        println!("group = \"{}\"", group.name);
        println!("Values:");
        for value in &group.values {
            println!("  {:#?}", value);
        }
    }

    // assert_eq!("main", groups["main"].name);

    // let opt: Option<_> = a["main"];

    // println!("{:#?}", lex.next());
}
