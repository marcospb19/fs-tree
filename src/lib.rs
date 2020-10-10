//! Representation of directory/file structure in file system to create, delete
//! or link.
//!
//! This crate is in a early development stage, we can only read file structures
//! for now.
//!
//! There's a lot left to be documented... there are some code in `examples/`
//! folder for now, it can give you a blurry image of what this crate is about.
//!
//! See source code for `File` and `FileType` structs, and the methods they
//! supply.
//!
//! # Performance note:
//! This might change, but this crate isn't intended to be the fastest one out
//! there, there is a lot to improve in terms of performance, however, we will
//! be more focused in nice error treatment instead of blazing thought the file
//! system and returning a `io::Result` for everything.
//!
//! # Alternatives:
//! If you don't want to create structures, but instead, just read directories,
//! I suggest you use `walkdir` instead.
//!
//! ---
//!
//! There's a crate in progress to make a human readable parser out of this
//! representation.
//!
//! TODO:
//! .from_text() method for File
//! .merge() method for File
//! FileType -> mode_t

// All items inside are `pub use`d
mod file;
mod file_type;
mod iter;

// This pub is redundant (as time of writing, because we already export each)
/// `FSError`, `FSErrorKind` and `FSResult`
pub mod error;
/// Exposed functions used by our modules
pub mod util;

pub use crate::{error::*, file::*, file_type::*, iter::*};
