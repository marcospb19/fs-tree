//! # Permissions crate
//! [![Crates.io](https://img.shields.io/crates/v/permissions.svg)](https://crates.io/crates/permissions)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/marcospb19/permissions/blob/main/LICENSE)
//! [![Docs.rs](https://docs.rs/permissions/badge.svg)](https://docs.rs/permissions)
//!
//! Useful filesystem queries for file permissions:
//! - [`is_executable`]
//! - [`is_readable`]
//! - [`is_writable`]
//! - [`is_removable`]
//!
//! See [`functions`].
//!
//! # Cross-platform
//! I tested this lib to work in `Unix` systems, I'm not sure about `Windows`
//! compatibility (PR welcome! if you can test it and update this section).
//!
//! # Examples:
//! ```rust
//! use permissions::*;
//!
//! fn main() -> std::io::Result<()> {
//!    println!("{:?}", is_readable("src/lib.rs")?);
//!    println!("{:?}", is_writable("src/lib.rs")?);
//!    println!("{:?}", is_executable("src/lib.rs")?);
//!    println!("{:?}", is_removable("src/lib.rs")?);
//!
//!    Ok(())
//! }
//! ```
//!
//! # More about it
//! For the 0.3 version I plan on adding an nicer `rwx` bitmask interface, if
//! you're interested, open an issue and I'll consider completing it sooner.
//!
//! I haven't finished 0.3 because I didn't needed it, I just needed this crate
//! to implement what's in 0.1 for other project of mine, that's why I'm waiting
//! for someone to ask me to implement it before I do so.
//!
//! I also want to ask what are the needs of other people for these features in
//! 0.3.
//!
//! Part of the code for `rwx` and `(Owner | Group | Other)` permissions
//! bitflags are already available at the project's repository.
//!
//! # Helping/Contributing:
//! It's easy to contribute to this crate, here are some options:
//!
//! - Share it to a friend.
//! - Help improve this README or other docs (even with little details).
//! - Open an issue or PR in the repository.
//! - Leave a star on GitHub.
//! - Use it!!!

pub mod functions;
pub use functions::*;
