// This should only compiles on unix systems

// use permissions::{Classes, ModeBits, PermissionBits};

// use std::fs::File;

// fn main() -> std::io::Result<()> {
//     let file = File::open("foo.txt")?;
//     // Troca isso por um wrapper
//     // let mut perms = file.metadata()?.permissions();
//     let permissions = PermissionBits::from_file(file)?;

//     if permissions.get(Owner, Execute) {
//         println!("Owner of file can execute");
//     }

//     if permissions.get(Other, Read | Write) {
//         println!("Other users can read and write to this file!");
//     }

//     // Let's change permissions with set and unset
//     let permissions = permissions
//         .unset(Owner, Execute)
//         .set(Group, Read | Write)
//         .reset(Other, Read);

//     perms.set_readonly(true);
//     file.set_permissions(perms)?;
//     Ok(())
// }

// Pending
// is set_readonly() == .unset(AllClasses, Write) ?????
fn main() {}
