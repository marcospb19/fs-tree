// use std::{fs::OpenOptions, path::Path};

// pub fn can_i_delete_it(path: impl AsRef<Path>) -> bool {
//     let path = path.as_ref();
//     let parent_dir: Option<_> = path.parent();

//     if let None = parent_dir {
//         return false;
//     }
//     let parent_dir = parent_dir.unwrap();

//     let file = OpenOptions::new()
//         .create(false)
//         .append(true)
//         .open(parent_dir);
//     let ok = file.is_ok();
//     ok
// }

// #[cfg(test)]
// mod tests {
//     #[allow(unused_imports)]
//     use std::{fs::OpenOptions, path::Path};
//     use unix_file_permissions::can_i_delete_it;

//     #[test]
//     fn asd() {
//         let foi = can_i_delete_it("/home/marcospb19");
//         // assert!(foi);
//         println!("{:?}", foi);

//         let file = OpenOptions::new().create(false).append(true).open("src");
//         println!("{:#?}", file);
//     }
// }
