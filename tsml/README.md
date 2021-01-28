# tree-file
Lexer and parser for new file format intended to describe a file tree structure.

Limitations: This won't support non-UTF-8 paths, they are supported in Unix systems, but supporting them would be against the intent of this crate.

RESTARTING!

# Progress
- [x] FileStructure crate implementation for file structure representation (other project link goes here)
- [x] API decisions
- [x] Syntax stabilization + examples skratch
- [x] Functional lexer
- [ ] Functional parser
- [ ] Error message pointing to line and column
- [ ] Fix lexer regex support to escaped and special characters
- [ ] Implement groups
- [ ] Implement flags support
- [ ] Improve this readme
- [ ] Remove all non-idiomatic panics
- [ ] Enum for types of flags (normal, parentInherited, and groupInherited)
- [ ] Nicer error messages, that say what was expected and where
- [ ] Documentation, lol
- [ ] Review where we accept whitespaces
- [ ] Rethink decision about non-UTF-8, or document it better (Documentation, lol)
- [ ] Document FileStructure too if you're using it as an dependency
- [ ] What about writing good docs?
- [ ] Better crazy RUSTC level error messages for the parser
- [ ] Clean some ugly code
- [ ] ++(docs++) Add examples field for each item in the public API
- [ ] Release 1.0
