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

pub mod file;
pub mod file_type;
pub mod iter;

pub use file::File;
pub use file_type::FileType;
pub use iter::{FilesIter, PathsIter};

// // This don't work as expected
// pub type File<T = ()> = file::File<T>;
// pub type FileType<T = ()> = file_type::FileType<T>;
// pub type PathsIter<'a, T = ()> = iter::PathsIter<'a, T>;
// pub type FilesIter<'a, T = ()> = iter::FilesIter<'a, T>;

/// Exposed functions that are used internally by this crate
pub mod util;

// Exposing error module and everything inside of it, redundantly
/// `FSError`, `FSErrorKind` and `FSResult`
pub mod error;
pub use crate::error::*;
