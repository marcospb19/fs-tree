# fs-tree

[`FsTree`] is a path [Trie] with an API focused on filesystem operations.

## Clarifications:

1. _Unix_ only.
2. This crate was transfered after `0.1.3`, and changed its purpose.
3. This crate refers to _"filesystem tree"_ as the result you get from recursively traversing files:
    - If you try traversing a single file, you get a single node.
    - If you try traversing a directories, you might get a large subtree (of nodes).
    - This is agnostic to the underlying filesystem (nothing to do with `ext4` or `btrfs`).
4. Check [Trie] if you both haven't met yet.

## Tree Layout:

A `FsTree` is a node with three possible file types:

```rust
use std::{collections::BTreeMap, path::PathBuf};

pub enum FsTree {
    Regular,
    Directory(TrieMap), // Recursive part
    Symlink(PathBuf),
}
//                                     ↓↓
pub type TrieMap = BTreeMap<PathBuf, FsTree>; // Recursive part
```

The root of the `FsTree` is **unnamed** (no filename/path), the "edges" to children are the
relative paths.

## Pitfall warning:

Like `std` functions, functions in this crate follow symlinks (and symlink chains), so you'll
never get a [`FsTree::Symlink(_)`] in your tree! If you want symlink-awareness, use the function
version with the `symlink` prefix ([`FsTree::read_at`] vs [`FsTree::symlink_read_at`]).

## Ways to construct a [`FsTree`]:

1. Read node/tree from path. ([`FsTree::symlink_read_at`])
2. Declare a `FsTree` literal. ([`tree!`])
3. Insert each node in an empty folder. ([`FsTree::new_dir`] + [`FsTree::insert`])
4. Parse from path text segments. ([`FsTree::from_path_text`])

## What you can do with a [`FsTree`]:

1. Traverse, query, and modify it.
2. Merge with another tree. ([`FsTree::try_merge`])
3. Write it to disk. ([`FsTree::write_at`])
4. Try loading a structural copy of it from a path. ([`FsTree::read_copy_at`])
5. (TODO) Compare with another `FsTree`, generating a DiffTree.
6. (TODO) Add entry API.

## Iterators:

See docs in the [`iter` module].

## Alternatives:
- Crate [`walkdir`](https://docs.rs/walkdir) - Better if you just need to iterate on
filesystem trees.
- Crate [`file_type_enum`](https://docs.rs/file_type_enum) - If you want a shallow type enum.
- Crate [`build-fs-tree`](https://crates.io/crates/build-fs-tree) - If you need to create a
filesystem tree from a YAML file.
    - The closest we got is creating a tree literal with [`tree!`](crate::tree), and writing
with [`FsTree::write_at`].

[Trie]: https://en.wikipedia.org/wiki/Trie
[`FsTree::from_path_text`]: https://docs.rs/fs-tree/latest/fs_tree/enum.FsTree.html#method.from_path_text
[`FsTree::insert`]: https://docs.rs/fs-tree/latest/fs_tree/enum.FsTree.html#method.insert
[`FsTree::new_dir`]: https://docs.rs/fs-tree/latest/fs_tree/enum.FsTree.html#method.new_dir
[`FsTree::read_at`]: https://docs.rs/fs-tree/latest/fs_tree/enum.FsTree.html#method.read_at
[`FsTree::read_copy_at`]: https://docs.rs/fs-tree/latest/fs_tree/enum.FsTree.html#method.read_copy_at
[`FsTree::Symlink(_)`]: https://docs.rs/fs-tree/latest/fs_tree/enum.FsTree.html#variant.Symlink
[`FsTree::symlink_read_at`]: https://docs.rs/fs-tree/latest/fs_tree/enum.FsTree.html#method.symlink_read_at
[`FsTree::try_merge`]: https://docs.rs/fs-tree/latest/fs_tree/enum.FsTree.html#method.try_merge
[`FsTree::write_at`]: https://docs.rs/fs-tree/latest/fs_tree/enum.FsTree.html#method.write_at
[`FsTree`]: https://docs.rs/fs-tree/latest/fs_tree/enum.FsTree.html
[`iter` module]: https://docs.rs/fs-tree/latest/fs_tree/iter/index.html
[`tree!`]: https://docs.rs/fs-tree/latest/fs_tree/macro.tree.html
