# tsml (beta)
TSML stands for Tree Structure Markup Language

It's a human-editable file format, in beta, the syntax is unstable.

At first the lexer was done by hand, but I switched to Logos, and did a parser myself for the tokens.

Limitation of this implementation: Doesn't support invalid UTF-8, which are allowed in Unix paths.

# Development stage
Where is this project at?

At the beginning, the focus for now is to just think about the design choices and stabilizing the syntax.

Optimizing code is not a priority.

# Progress
- [x] FileStructure crate implementation for file structure representation (other project link goes here)
- [x] API decisions
- [x] Syntax stabilization + examples skratch
- [x] Functional lexer
- [x] Functional parser
- [ ] Error message pointing to line and column
- [ ] Fix lexer regex support to escaped and special characters
- [x] Implement groups
- [x] Implement flags support
- [ ] Improve this readme
- [ ] Remove all non-idiomatic panics
- [x] Enum for types of flags (normal, parentInherited, and groupInherited)
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
