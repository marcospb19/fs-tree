use crate::error::*;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DotfileGroup {
    // starting_path: PathBuf, files: Vec<File>,
}

impl DotfileGroup {
    pub fn new<T: AsRef<Path>>(_path: T) -> Result<Self> {
        let _a = PathBuf::new().metadata()?;
        Ok(DotfileGroup {})
    }
}
