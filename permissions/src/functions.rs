//! Check for permissions at a path using [`access_syscall`].
//!
//! Having permission of reading, writing, executing or deleting a file does not
//! guarantee success in doing so, it is unlikely but IO can fail.
//!
//! Also be careful with [`TOCTOU race conditions`], when you have outdated file
//! system information that has changed since the last check.
//!
//! [`TOCTOU race conditions`]: https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use

use std::{io, os::raw::c_int, path::Path};

/// Check if current process has permission to remove file.
///
/// That is, if the current process has permission of `write` to the parent
/// directory.
///
/// Returns `false` if there's no parent directory (because can't delete the
/// system's root).
///
/// # Errors
///
/// - If [`Path::canonicalize`] fails.
/// - Same as [`access_syscall`].
///
/// # Examples
/// ```
/// use permissions::is_removable;
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     println!("{:?}", is_removable("src/lib.rs")?);
///     println!("{:?}", is_removable("/root")?);
///     println!("{:?}", is_removable("/")?);
///
///     // May return `Err(kind: PermissionDenied)`
///     // println!("{:?}", is_removable("/root/any")?);
///
///     Ok(())
/// }
/// ```
/// [`Path::canonicalize`]: std::path::Path::canonicalize
pub fn is_removable(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref().canonicalize()?;
    let parent = match path.parent() {
        // Cannot delete '/' (root)
        None => return Ok(false),
        Some(parent) => parent,
    };

    access_syscall(&parent, libc::W_OK)
}

/// Check if current process has permission to read.
///
/// # Errors
/// Same as [`access_syscall`].
///
/// # Examples
/// ```
/// use permissions::is_readable;
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     println!("{:?}", is_readable("src/lib.rs")?);
///     println!("{:?}", is_readable("/root")?);
///     println!("{:?}", is_readable("/")?);
///
///     // may return `Err(kind: PermissionDenied)`
///     // println!("{:?}", is_readable("/root/any")?);
///
///     Ok(())
/// }
/// ```
pub fn is_readable(path: impl AsRef<Path>) -> io::Result<bool> {
    access_syscall(&path.as_ref(), libc::R_OK)
}

/// Check if current process has permission to write.
///
/// # Errors
/// Same as [`access_syscall`].
///
/// # Examples
/// ```
/// use permissions::is_writable;
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     println!("{:?}", is_writable("src/lib.rs")?);
///     println!("{:?}", is_writable("/root")?);
///     println!("{:?}", is_writable("/")?);
///
///     // may return `Err(kind: PermissionDenied)`
///     // println!("{:?}", is_writable("/root/any")?);
///
///     Ok(())
/// }
/// ```
pub fn is_writable(path: impl AsRef<Path>) -> io::Result<bool> {
    access_syscall(&path.as_ref(), libc::W_OK)
}

/// Check if current process has permission to execute.
///
/// If `path` points to a directory, you'll be checking if you have the right to
/// enter it.
///
/// # Errors
/// Same as [`access_syscall`].
///
/// # Examples
/// ```
/// use permissions::is_executable;
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     assert!(is_executable("/usr/bin/cat")?);
///     assert!(is_executable("/")?);
///     assert!(is_executable("src/")?);
///     assert!(!is_executable("src/lib.rs")?);
///     assert!(!is_executable("/root")?);
///
///     // may return `Err(kind: PermissionDenied)`
///     // println!("{:?}", is_executable("/root/any")?);
///
///     Ok(())
/// }
/// ```
pub fn is_executable(path: impl AsRef<Path>) -> io::Result<bool> {
    access_syscall(&path.as_ref(), libc::X_OK)
}

/// Safe wrapper to the `libc::access` syscall.
///
/// See [`access man page`].
///
/// Used by:
/// - [`is_removable`]
/// - [`is_readable`]
/// - [`is_writable`]
/// - [`is_executable`]
///
/// This function requires a bitmask made of:
/// - [`libc::R_OK`] _(Read)_
/// - [`libc::W_OK`] _(Write)_
/// - [`libc::X_OK`] _(Execute)_
///
/// To check for each given `rwx` permission, or:
/// - [`libc::F_OK`] _(File exists)_
///
/// Otherwise, the function fails with [`Err(kind:
/// InvalidInput)`](std::io::ErrorKind::InvalidInput)
///
/// # Examples:
///
/// ```
/// use permissions::access_syscall;
/// use libc::{R_OK, W_OK, X_OK, F_OK};
///
/// fn main() -> std::io::Result<()> {
///     assert!(access_syscall("src/lib.rs", R_OK | W_OK)?);
///     assert!(access_syscall("/", R_OK | X_OK)?);
///     assert!(access_syscall(".", F_OK)?);
///
///     assert!(!access_syscall("src/lib.rs", X_OK)?);
///     assert!(!access_syscall("/root", W_OK)?);
///
///     Ok(())
/// }
/// ```
///
/// # Errors
/// See [`access man page`].
///
/// [`io::Error`]: std::io::Error
/// [`access man page`]: https://man7.org/linux/man-pages/man2/access.2.html
pub fn access_syscall(path: impl AsRef<Path>, mode_mask: c_int) -> io::Result<bool> {
    let path = path.as_ref();

    // Syscall for unix and windows systems: https://stackoverflow.com/a/59224987/9982477
    let mut buf = Vec::new();
    let buf_ptr;
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        buf.extend(path.as_os_str().as_bytes());
        buf.push(0u8);
        buf_ptr = buf.as_ptr() as *const libc::c_char;
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        buf.extend(
            path.as_os_str()
                .encode_wide()
                .chain(Some(0u16))
                .map(|b| {
                    let b = b.to_ne_bytes();
                    b.get(0).map(|s| *s).into_iter().chain(b.get(1).map(|s| *s))
                })
                .flatten(),
        );
        buf_ptr = buf.as_ptr() as *const libc::c_wchar_t;
    }

    let access_return_code = unsafe { libc::access(buf_ptr, mode_mask) };

    match access_return_code {
        0 => Ok(true),
        _ => {
            let err = io::Error::last_os_error();
            if err.raw_os_error().unwrap() == libc::EACCES {
                Ok(false) // Ok, no permission to delete
            } else {
                Err(err) // Syscall error
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::{F_OK, R_OK, W_OK, X_OK};

    #[test]
    fn test_access_syscall() {
        assert!(access_syscall("src/", F_OK | R_OK | W_OK | X_OK).unwrap());
        assert!(access_syscall("src/", F_OK).unwrap());
        assert!(access_syscall("src/", R_OK).unwrap());
        assert!(access_syscall("src/", W_OK).unwrap());
        assert!(access_syscall("src/", X_OK).unwrap());

        assert!(!access_syscall("src/lib.rs", X_OK).unwrap());
        assert!(!access_syscall("src/lib.rs", X_OK | R_OK).unwrap());
        assert!(!access_syscall("src/lib.rs", X_OK | W_OK).unwrap());
        assert!(!access_syscall("src/lib.rs", X_OK | F_OK).unwrap());

        assert!(access_syscall("path_doesnt_exist/", F_OK).is_err()); // invalid path
        assert!(access_syscall("src/", 0b11111).is_err()); // invalid number

        assert!(!access_syscall("src/lib.rs", X_OK).unwrap());
        assert!(!is_removable("/").unwrap());
    }
}
