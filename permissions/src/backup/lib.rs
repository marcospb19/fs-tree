//! Provides a set of useful permission functions, and friendlier bits
//! permission management for `unix`.
//!
//! # Sections:
//! There are two sections, the set of permission functions, and the `unix`
//! specific section.
//!
//! # Cross-platform functions section:
//! Set of useful functions:
//! ```ignore
//! is_executable
//! is_readable
//! is_writable
//! is_removable
//! ```
//!
//! ## Example:
//! ```rust
//! use permissions::is_removable;
//!
//! use std::{fs, io};
//!
//! fn can_clear_trash_bin() -> io::Result<bool> {
//!    for entry in fs::read_dir("~/.local/share/Trash/files/")? {
//!        let entry = entry?;
//!
//!        if !is_removable(entry.path())? {
//!            return Ok(false);
//!        }
//!    }
//!    Ok(true)
//! }
//! ```
//!
//! # Unix specific modules:
//!
//! This crate also offers ModeBits for friendly `rwx` permissions, yet to be
//! documented

pub mod prelude;

// Import macro from bitflags, it's used to declare Classes and ModeBits.
#[macro_use]
extern crate bitflags;

mod classes;
mod functions;
mod mode_bits;
mod permission_bits;

/// Classes of permissions in `Unix` _(Owner, Group, Other)_
pub use classes::Classes;
/// Very useful permission check functions.
pub use functions::*;
/// Mode permission bits _(Read, Write, Execute)_.
pub use mode_bits::ModeBits;
/// PermissionBits...
pub use permission_bits::PermissionBits;
