# Dotao

Not ready for use, starting to function

# stuff

Clone the repo
```sh
git clone https://github.com/marcospb19/dotao
```

For testing/developing, I recommend you installing it and running on another folder
```sh
cargo install --path .
cargo install --path . --debug           # Instead of release
cargo install --path . --debug --offline # Skip crates.io update
```

Because you want to be testing `dotao` inside of a git repository at `~/dotfiles`.

Then you can run

```sh
dotao help
dotao init
dotao add [folders...]
dotao link
```

But those folders added need to follow the mfin' desired structure.
