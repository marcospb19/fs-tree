//! # Permissions crate
//! [![Crates.io](https://img.shields.io/crates/v/permissions.svg)](https://crates.io/crates/permissions)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/marcospb19/dotao/blob/master/LICENSE)
//! [![Docs.rs](https://docs.rs/permissions/badge.svg)](https://docs.rs/permissions)
//!
//! Useful filesystem queries for file permissions:
//!
//! See [`functions`](https://docs.rs/permissions/latest/permissions/functions/index.html).
//! - [`is_executable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_executable.html)
//! - [`is_readable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_readable.html)
//! - [`is_writable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_writable.html)
//! - [`is_removable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_removable.html)
//! - [`is_creatable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_creatable.html)
//!
//! # Cross-platform
//! This is expected to work with `Windows` as well, but it was only tested in `Unix` machines.
//! (PR welcome! Need someone to test it and update this section).
//!
//! # Examples:
//! ```
//! use permissions::*;
//!
//! fn main() -> std::io::Result<()> {
//!    // Functions accept `AsRef<Path>`
//!    assert!(is_readable("src/")?);
//!    assert!(is_writable("src/")?);
//!    assert!(is_writable("src/lib.rs")?);
//!    assert!(is_executable("/usr/bin/cat")?);
//!    assert!(is_removable("src/lib.rs")?);
//!    assert!(is_creatable("src/file.rs")?);
//!
//!    Ok(())
//! }
//! ```
//!
//! # Future
//! For this crate I plan on adding a nicer and convenient `rwx` bitmask interface, in the 0.4
//! version.
//!
//! If you're interested in this implemented, open an issue and I'll try to complete it sooner.
//!
//! I haven't finished it yet beucase I've never needed this functionality, the existing functions
//! were enough for my use cases.
//!
//! (Part of the code for `rwx` and `(Owner | Group | Other)` permissions bitflags are already
//! available at the project's repository)
//!
//! # Helping/Contributing:
//! It's easy to contribute to this crate, here are some options:
//! - Share it to a friend.
//! - Help improve this README or other docs (even little details).
//! - Open an issue or PR in the repository.
//! - Use it and give feedback.
//! - Suggest how to improve.

#![warn(missing_docs)]

pub mod functions;
pub use functions::*;
