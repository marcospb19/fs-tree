//! Work in progress!

//! We do deal a lot with symbolic links, so some Dotao functions accept a
//! "follow_symlink(s)" argument that is essential.
//!
//! For example, when we are reading dotfiles from the user dotfiles repository,
//! we want to make it able for they to use symlinks there, is their choice, so
//! the program will follow the symlinks. But when we are scanning in the home
//! directory for the structure to link everything, we must not follow symlinks,
//! we must see if the files are itself links, and then treat them.

/// Mainly DotfileGroup, which is Dotao representation of a dotfile.
pub mod dotfiles;
/// dotao::error::Result<T, DotaoError> !!!!
pub mod error;
/// Our implementation of File and FileType
pub mod file;
/// Trait to link dotfiles, implemented for DotfileGroup
pub mod link;
