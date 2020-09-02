use std::path::Path;

#[derive(Debug)]
pub struct DotfileGroup {
    // starting_path: PathBuf, files: Vec<File>,
}

impl DotfileGroup {
    pub fn new<T: AsRef<Path>>(_path: T) -> Self {
        DotfileGroup {}
    }
}
