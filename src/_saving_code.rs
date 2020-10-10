// So, we will save this code for PathsIter in case we change the structure of
// `File` and `FileType` in the future, so that the paths aren't full relative
// paths, but instead, just the last component.
//
// Trade-off:
// Pro:
// - This would make memory consumption for large file structures *incredibly*
//   smaller
//
// Cons:
// - It wouldn't be possible to access full relative path through
//   `file_reference.path`, but instead, only with iterators that "build" the
//   path while iterating

#[derive(Debug, Clone)]
pub struct PathsIter<'a> {
    // We will make a lot of pushs and pops in this path from each segment of path
    current_path: PathBuf,
    last_depth: usize,
    file_iter: FilesIter<'a>,
    // options
    only_show_last_segment: bool,
}

impl<'a> PathsIter<'a> {
    pub fn new(file_iter: FilesIter<'a>) -> Self {
        Self {
            file_iter,
            last_depth: 0,
            current_path: PathBuf::new(),
            only_show_last_segment: false,
        }
    }

    pub fn only_show_last_segment(mut self, arg: bool) -> Self {
        self.only_show_last_segment = arg;
        self
    }

    pub fn depth(&self) -> usize {
        self.file_iter.depth()
    }
}

impl Iterator for PathsIter<'_> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let file = self.file_iter.next()?;
        let current_depth = self.file_iter.depth();

        let result: &PathBuf = if self.only_show_last_segment {
            &file.path
        } else {
            // Let's prepare self.current_path based on depths change and file.path
            // About `self.current_path.pop` and `self.current_path.push(&file.path)`
            //
            // Based on the depth difference between last run and this one:
            // < , pop twice, and push once
            // ==, pop and push once
            // > , push once

            if current_depth < self.last_depth {
                self.current_path.pop();
            }
            if current_depth <= self.last_depth {
                self.current_path.pop();
            }
            self.current_path.push(&file.path);
            &self.current_path
        };

        // Update last_depth before returning
        self.last_depth = current_depth;
        Some(result.clone())
    }
}
