use std::{fs::OpenOptions, path::Path};

pub fn can_i_delete_it(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    let parent = path.parent().unwrap_or_else(|| {
        return false;
    });
    if path == Path::new("/") {
        unimplemented!();
    }
    let file = OpenOptions::new().create(false).append(true).open(path);
    let ok = file.is_ok();
    ok
}

#[cfg(test)]
mod tests {
    #[test]
    fn asd() {
        use std::path::PathBuf;
        // can_i_delete_it("/")
        let path: PathBuf = "/proc/self".into();
        println!("q {:?}", path.canonicalize());
        let result = path.symlink_metadata().unwrap();
        println!("sera que eh {:#?}", result.file_type().is_symlink());
    }
}
