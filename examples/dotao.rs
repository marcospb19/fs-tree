// use tree_file::{Group, Groups};

// use std::fs;

// // impl std::ops::Index<&str> for Vec<Group> {
// //     type Output = usize;

// //     fn index(&self, _: Self) -> Self::Output {
// //         10
// //     }
// // }

// fn main() {
//     let text = fs::read_to_string("examples/dotao.tree").expect("Missing
// examples/dotao.tree");     let groups = Groups::from_text(&text);

//     for (group, values) in &groups {
//         println!("group = \"{}\"", group);
//         println!("Values:");
//         for value in &values {
//             println!("  {:#?}", value);
//         }
//     }

//     // let oi: i32 = vec![123, 123, 123].iter();

//     println!("oi");

//     // assert_eq!("main", groups["main"].name);

//     // let opt: Option<_> = a["main"];

//     // println!("{:#?}", lex.next());
// }
