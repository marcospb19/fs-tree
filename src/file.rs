use std::path::PathBuf;

#[derive(Debug)]
pub struct File {
    path: PathBuf,
    file_type: FileType,
}

#[derive(Debug)]
pub enum FileType {
    File,
    Directory { children: Vec<File> },
    SymbolicLink { points_to: PathBuf },
}

#[derive(Debug)]
pub enum FlatFileType {
    File,
    Directory,
    SymbolicLink,
}
