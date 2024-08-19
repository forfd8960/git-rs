# Change Log


## 24-08-19

* Add Add Sub-Command
* Add Commit Sub-Command

```sh
cargo run -- help add
   Compiling git-rs v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.32s
     Running `target/debug/git-rs help add`
add files into index

Usage: git-rs add [OPTIONS] --path <PATH_SPEC>

Options:
  -p, --path <PATH_SPEC>  Files to add content from
  -a, --all               If no <pathspec> is given when -A option is used, all files in the entire working tree are updated
  -h, --help              Print help
```

```sh
cargo run -- help commit
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/git-rs help commit`
Create a new commit containing the current contents of the index and the given log message describing the changes

Usage: git-rs commit [OPTIONS] --message <MESSAGE>

Options:
  -a, --all                Tell the command to automatically stage files that have been modified and deleted, but new files you have not told Git about are not affected
  -m, --message <MESSAGE>  Record changes to the repository
  -e, --edit               The message taken from file with -F, command line with -m, and from commit object with -C are usually used as the commit log message unmodified. This option lets you further edit the message taken from these sources
  -n, --no-edit            Use the selected commit message without launching an editor
      --amend
  -h, --help               Print help
```

## 24-08-18

* Set up the project.
* Add command module.
* Add init command.

```sh
cargo run -- help init
   Compiling git-rs v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/git-rs help init`
init a git repo

Usage: git-rs init [OPTIONS] --template <TEMPLATE> --initial-branch <BRANCH>

Options:
  -q, --quiet                          Only print error and warning messages; all other output will be suppressed
      --object-format <OBJECT_FORMAT>  Specify the given object <format> (hash algorithm) for the repository. The valid values are sha1 and (if enabled) sha256. sha1 is the default [default: sha1]
      --ref-format <REF_FORMAT>        [default: files]
      --template <TEMPLATE>            Specify the directory from which templates will be used
  -b, --initial-branch <BRANCH>        Use <branch-name> for the initial branch in the newly created repository. If not specified, fall back to the default name
  -h, --help                           Print help
```
