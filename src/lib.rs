//! Representation of directory/file structure in filesystem to create, delete
//! or link.
//!
//! This crate is in a early development stage, we can only read file structures
//! for now.
//!
//! Yet to be documented... see `File` and `FileType` structs, and the `from*`
//! methods they suply.

mod error;
mod file;
mod file_type;
mod util;

pub use crate::{error::*, file::*, file_type::*, util::*};
