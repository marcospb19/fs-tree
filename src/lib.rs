//! Representation of directory/file structure in filesystem to create, delete
//! or link.
//!
//! For short, FS?.
//!
//! This crate is in a early development stage, we can only read file structures
//! for now.
//!
//! Yet to be documented... see `File` and `FileType` structs, and the `from*`
//! methods they suply.
//! ### Extra:
//! Why did I named this file structure if it is a filesystem structure?
//!
//! This crate has no intent to be the fastest one, but to be very usable, we
//! will make excessive checks upfront to try to give a better error treatment

// Itens inside are exposed
mod file;
mod file_type;
mod iter;

pub use crate::{file::*, file_type::*, iter::*};
/// `FSError` and `FSErrorKind`
pub mod error;
/// Exposed functions used by our modules
pub mod util;
