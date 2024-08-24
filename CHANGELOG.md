# Change Log

## 24-08-24 parse git index

```sh
✗ ./git-rs add --path .
SimpleGit { command: Add(AddOpts { path_spec: ".", all: false }) }
add file options: AddOpts { path_spec: ".", all: false }
version: 2
Index { version: 2, entries: [], cache: Tree { entries: [] } }
```

## 24-08-24

```sh
./git-rs init
SimpleGit { command: Init(InitOpts { quiet: false, object_format: Some("sha1"), ref_format: Some("files"), template: None, branch: None }) }
init repo optiond: InitOpts { quiet: false, object_format: Some("sha1"), ref_format: Some("files"), template: None, branch: None }
create: how-git-works1/.git/HEAD, and write: ref: refs/heads/main

create: how-git-works1/.git/config, and write:
create: how-git-works1/.git/description, and write:
```

## 24-08-22

```sh
cargo run -- help branch
   Compiling git-rs v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 12.99s
     Running `target/debug/git-rs help branch`
git-branch - List, create, or delete branches

Usage: git-rs branch --delete <DELETE> --move <MOVE_BRANCH> --copy <COPY> --list <LIST>

Options:
  -d, --delete <DELETE>     delete a branch
  -m, --move <MOVE_BRANCH>  Move/rename a branch, together with its config and reflog
  -c, --copy <COPY>         Copy a branch, together with its config and reflog
  -l, --list <LIST>         List branches. With optional <pattern>..., e.g. git branch --list 'maint-*', list only the branches that match the pattern(s)
  -h, --help                Print help

```

## 24-08-21

```sh
git-rs git:(main) ✗ cargo run -- help log
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/git-rs help log`
show commit logs

Usage: git-rs log [OPTIONS] --number <NUMBER> --max-count <MAX_COUNT> --skip <SKIP> --since <SINCE> --after <AFTER> --until <UNTIL> --before <BEFORE> --author <AUTHOR> --committer <COMMITTER> --pretty <PRETTY> --format <FORMAT>

Options:
  -o, --oneline                This is a shorthand for "--pretty=oneline --abbrev-commit" used together
  -n, --number <NUMBER>
      --max-count <MAX_COUNT>  Limit the number of commits to output
      --skip <SKIP>            Skip number commits before starting to show the commit output
      --since <SINCE>          Show commits more recent than a specific date
      --after <AFTER>          Show commits more recent than a specific date
      --until <UNTIL>          Show commits older than a specific date
      --before <BEFORE>        Show commits older than a specific date
      --author <AUTHOR>        --author=<pattern>, --committer=<pattern>
      --committer <COMMITTER>  commiter
      --pretty <PRETTY>        Pretty-print the contents of the commit logs in a given format, where <format> can be one of oneline,
      --format <FORMAT>        Pretty-print the contents of the commit logs in a given format, where <format> can be one of oneline,
  -h, --help                   Print help
```

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
