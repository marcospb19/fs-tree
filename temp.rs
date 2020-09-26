
//     // We'll create a deque and vec of &File, that we'll use to check each
// file     // entry.
//     //
//     // The deque will contain files left to check and pass to the vec
//     //
//     // The vec contains itens that are ok for linking, checked and OK to
// proceed     //
//     // The order of the deque is weirdly specific, like an DFS traversal that
//     // sometimes prioritizes files over directories, the intent here is to
// make the     // error messages more intuitive
//     let mut deque = dotfile_group.deque_from_file_references();
//     let mut link_information = LinkInformation::new();

//     // Please document this im so tired right now that I can't
//     while let Some(file) = deque.pop_front() {
//         let target_path = home_path.join(&file.path);
//         let source_path = dotfile_group
//             .starting_path
//             .join(&file.path)
//             .canonicalize()
//             .expect("Error resolving path of file, should not fail!");

//         if !target_path.exists() {
//             link_information.files_to_link.push(&file);
//             continue;
//         }

//         // Type for both source and target
//         let source_file_type = &file.file_type;
//         let target_file_type = FileType::from_path_shallow(&target_path,
// false)?;         println!(" --- {} {:?}", &target_path.display(),
// target_file_type);

//         // let files_to_delete = vec![];
//         // // This code was deleted!
//         // let can_be_deleted =
//         //     can_i_delete_it(&target_path).map_err(|err|
// DotaoError::LinkError {         //         from: source_path,
//         //         to: target_path,
//         //         source: err,
//         //     })?;

//         use FileType::*;

//         if let SymbolicLink { target_path } = target_file_type {
//             unimplemented!();
//         }

//         match source_file_type {
//             // Maybe this check shouldn't be here, but I wanna be 100% sure
// that this won't             // happen at this point it can't be a symlink O.o
//             SymbolicLink { .. } => {
//                 panic!("DevErr: We shouldn't be trying to create symlinks of
// symlinks!");             },

//             File => {
//                 let result = link_check_for_regular_file(&mut
// link_information, target_file_type)                     .expect("expect");
//                 continue;
//             },

//             Directory { .. } => {
//                 let result = link_check_for_directory(&mut link_information,
// target_file_type)                     .expect("askndnajndjasd");
//                  continue;
//             },
//             // (_, SymbolicLink { target_path }) => {
//             //     if true {
//             //         // if temp_target == source_path {
//             //         continue; // Ok, already in place
//             //     } else {
//             //         // Now we need to deal with it
//             //         if link_behavior.overwrite_symbolic_links {
//             //             // files_to_delete.push(&file);
//             //         } else {
//             //             eprintln!(
//             //                 "Problem, we found this symlink, but it points
// to another place: '{}' \             //                  -> '{}'",
//             //                 target_path.display(),
//             //                 target_path /* temp_target */
//             //                     .display()
//             //             );
//             //         }
//             //     }
//             //     if link_behavior.overwrite_symbolic_links {}
//             // },

//             /* Other than overwriting, we can check the size of the file, and
// then
//              * it's contents, to see if it is the exact
//              * same as the source one, if so, link
//              * if a custom option is already set yoooo */
//             /*         eprintln!("Encountered a file at {}, exiting.",
//              * target_path.display());         process::exit(1);
//              * } else {
//              * }
//              * }, */
//             /* (_, File) => {
//              * // Other than overwriting, we can check the size of the file,
//                and then
//              * // it's contents, to see if it is the exact
//              * // same as the source one, if so, link
//              * // if a custom option is already set yoooo */
//             /*     eprintln!("Encountered a file at {}, exiting.",
// target_path.display());
//              * process::exit(1);
//              * }, */
//             /* (Directory { .. }, File) => {
//              * eprintln!(
//              * "Found file instead of directory: '{}'",
//              * target_path.display()
//              * );
//              * }, */
//         } // match
//     } // for

//     // Check errors
//     // if error...

//     let mut link_counter = 0;

//     // if paths_to_link.len() == 0 {
//     //     println!("Nothing to do.");
//     //     return Ok(link_counter);
//     // }

//     // println!("{:#?}", paths_to_link);

//     // for path in paths_to_link {
//     //     let source_path = dotfile_group
//     //         .starting_path
//     //         .join(path)
//     //         .canonicalize()
//     //         .expect("Error resolving path");
//     //     let target_path = home_path.join(path);
//     //     println!("source = {}", source_path.display());
//     //     println!("target = {}", target_path.display());
//     //     let result = symlink_with_checks(source_path, target_path);
//     //     link_counter += 1;
//     //     println!("{:#?}", result);
//     // }

//     Ok(link_counter)
// }




/// `DotfileGroup` represents a folder with a list of files (dotfiles) inside of
/// it.
///
/// These dotfiles can be files, or directories, and each directory is a tree,
/// here is a example with `i3`.
///
/// ```sh
/// cd ~/dotfiles
/// dotao i3/
/// ```
///
/// When the user types this in the terminal, we'll create:
/// ```ignore
/// DotfileGroup {
///     starting_path: "i3",
///     files: vec![],
/// }
/// ```
///
/// Then the files will be filled with everything that is inside of the i3
/// folder, recursively, following symlinks
///
/// So, the method that links the DotfileGroup will panic if you let a
/// FileType::SymbolicLink inside of the file tree, this may change in the
/// future, ok?
///
/// For now just keep in mind that it does not make sense to have a symlink
/// there


/*
pub fn show(&self) {
    let a = self.starting_path.file_name().unwrap().to_string_lossy();

    println!("[{}]", a);
    for file in &self.files {
        DotfileGroup::show_rec(0, file);
    }
}

fn show_rec(level: u32, file: &File) {
    let a = &file.path /* .file_name().unwrap().to_string_lossy() */ ;

    let mut b = String::new();
    b.push_str("asjndasjnd");
    // println!("{:?}", b);

    for _ in 0..level {
        print!("    ");
    }
    if let FileType::Directory { children } = &file.file_type {
        println!("\"{}\": [", a.display());

        for file in children {
            DotfileGroup::show_rec(level + 1, &file);
        }
        for _ in 0..level {
            print!("    ");
        }
        println!("]");
    } else {
        println!("\"{}\",", a.display());
    }
}
*/
