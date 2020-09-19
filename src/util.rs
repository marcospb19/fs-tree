use std::{ffi::CString, io, path::Path};

pub fn can_i_delete_it(path: impl AsRef<Path>) -> io::Result<bool> {
    let parent = match path.as_ref().parent() {
        Some(parent) => parent,
        None => return Ok(false), // Cannot delete '/' (root)
    };

    let bytes: Vec<u8> = parent.to_str().unwrap().bytes().collect();
    let cstring = CString::new(bytes).unwrap();
    let result = unsafe { libc::access(cstring.as_ptr(), libc::W_OK) };

    if result == 0 {
        Ok(true) // Permission
    } else {
        assert!(result == -1);

        // From errno
        let err = io::Error::last_os_error();
        if err.raw_os_error().unwrap() == libc::EACCES {
            Ok(false) // No permission
        } else {
            Err(err) // Error while trying to gather permission
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn a() {
//         // sim
//         assert!(can_i_delete_it("src/") == true);
//         assert!(can_i_delete_it("src/util.rs") == true);
//         // nao
//         assert!(can_i_delete_it("/") == false);
//         assert!(can_i_delete_it("/root") == false);
//     }
// }
