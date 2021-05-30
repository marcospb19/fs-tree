# THIS README IS JUST A SCRATCH, TOOL IS NOT FINISHED.

# DOTÃO

`dotao` is a modern dotfiles manager that keeps you in control.

A dotfiles manager is a tool that helps organizing your _dotfiles_ (configuration files), making it easy to backup, share, remove and reapply the settings.

- [About](#About)
- [Installation](#Installation)
- [Usage](#Usage)
  - [Creating](#CREATING)
  - [Importing](#IMPORTING)
  - [Linking](#LINKING)
  - [Applying To Other Systems](#APPLYING-TO-OTHER-SYSTEMS)
- [Wiki](#Wiki)

# ABOUT

It is designed to work nicely with `git`, because you can:

- Monitor and revert file changes.
- Change history is compressed.
- Upload backups.
- Share dotfiles with friends.

However, using it with `git` is **optional**.

---

`dotao` uses a `dotao.tsml`:

- It keeps track of your dotfiles folder.
- Allows for some specific file and folder configuration.
- Allows for system-specific conditional layouts.
- Is human readable and editable.

This file is updated automatically by commands, you don't have to edit manually, but you can.

---

`dotao` uses symlinks to:

- Link imported files to their original location.
- You can still use them as if they were there.

# INSTALLATION

## Installing the binary

Run this script.
```sh
curl -s https://raw.githubusercontent.com/marcospb19/dotao/master/install.sh | sh
```

## Compiling from source

[Wiki - Compiling from source.](https://github.com/marcospb19/dotao/wiki/Compiling-from-source)

# USAGE

There are three main steps for using `dotao`:

1. Creating a new `~/dotfiles/` folder.
    - Containing a `dotao.tsml` file.
2. Importing your files into `~/dotfiles/`.
    - Moving files and updating `dotao.tsml` accordingly.
3. Linking imported files.
    - Creating symlinks to their original location.

Commands used in this tutorial for each step:

1. `dotao init`
2. `dotao import`
3. `dotao link`

The second and third steps are repeatable.

## Creating

In your user's home directory, create `dotfiles/` and enter it.

```sh
mkdir dotfiles
cd dotfiles
```

Now ask `dotao` to create a `dotao.tsml` configuration file.

```sh
dotao init
```

## Importing

Here's an example with `zsh` configuration files, to show how to easily import files to your folder.

`zsh` files usually stay in the home directory:

```c
~/.zshrc
~/.aliases
~/.functions
~/.profile
```

In this step, we will use the `import` command to:
- Create a `zsh` folder.
- Move each file.
- Update `dotao.tsml` with files info.

The folder will look like:

![Showing the tree directory structure, dotfiles/zsh contains 4 the files inside](https://i.imgur.com/tN65NZT.png)

Files moved:

```c
~/.aliases     ->  ~/dotfiles/zsh/.aliases
~/.functions  ->  ~/dotfiles/zsh/.functions
~/.profile   ->  ~/dotfiles/zsh/.profile
~/.zshrc    ->  ~/dotfiles/zsh/.zshrc
```

The command used:

```sh
dotao import zsh ~/.aliases ~/.functions ~/.profile ~/.zshrc
```

## Linking

`dotao.tsml` holds all the information needed to create the links.

So let's link it!

```sh
dotao link
```

Now your files are accessible by your applications at the same place just like before.

(Hint: before linking, run `dotao status` to see a linkage report.)

## Applying to other systems

Now that your files are inside of one folder, let's assume that you have uploaded to `github`.

Now in another system, clone your dotfiles and enter the folder:
```sh
git clone https://github.com/marcospb19/dotfiles # Use your URL
cd dotfiles
```

Inside of the folder, run `dotao status` to see if there are any conflicts:
```sh
dotao status
```

Once you have solved every conflict:
```sh
dotao link
```

Done, all your configs applied in another system.

# COMMANDS

`dotao` usage is all based on commands (similar to how the `git` CLI works).

```sh
dotao COMMAND ...
```

Commands may accept specific subcommands, arguments and flags.

## `dotao init`
Creates a `dotao.tsml` file, initiating dotfiles folder.

## `dotao update`
TODO INFO

## `dotao import`

Creates the necessary folders inside of `~/dotfiles/`, moves the files, and updates `dotao.tsml`.

So `dotao import` is just a convenient command that merges functionality from 3 other commands:

1. `mkdir`
2. `mv`
3. `dotao update`

## `dotao status`

Reads `dotao.tsml`, scans your dotfiles folder, and displays a report of:

- Is there something left to link?
- Is any file missing?
- Is there a file already in the target path?
  - Is it a file or folder?
  - If it is already a link, but points to the wrong place
- Are multiple files pointing to a same conflicting location?

Much like `git status` does, it can be used at any moment.

# WIKI

Check [our wiki](https://github.com/marcospb19/dotao/wiki).
