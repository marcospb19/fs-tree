**This is a scratch of the readme, the features aren't implemented yet.**

# Dotao
A modern dotfiles manager made for everyone.

Dotao tries to be a powerful, while also keeping the user in control.

# Why? (backstory)
I always liked to customize my text editor and system preferences to fit my needs.

However, every time I installed another system, I had to configure everything again, or copy files manually from a backup.

The problems are:
 - Making manual backups is forgettable and also painful.
 - Copying each backed up file to the correct position requires effort and memorization.
 - Both operations become unreliable as the number of files in different directories increase to dozens or hundreds.

Then I started using `stow`, an util made in 1993, it solves the problem by moving every file to a single place, and then making symbolic links to each original location.

This came very handy as now I could versionate my configuration files using `git`, so I could reverse my text editor changes at any time, however, `stow` is very limited.

Dotao is 100% inspired by `stow`.

# Installation
There are no pré-compiled binaries yet, because we haven't realeased a stable release of `dotao`, so these instructions are targeted ONLY to contributors.

Clone the repo
```sh
git clone https://github.com/marcospb19/dotao
cd dotao
```

For testing it in the CLI, I recommend you running `cargo install` and then testing it in another folder, like `~/dotfiles`.
```sh
cargo install --path . --debug
cargo install --path . --debug --offline # Skip crates.io update
```

Then you can run

```sh
dotao help
dotao init
dotao add [folders...]
dotao link
```

Following the guide.
