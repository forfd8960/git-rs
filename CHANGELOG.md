# Change Log

## 2408-26

* Fix compute sha1 hash
* Fix write index back to disk

```sh
> git init
Initialized empty Git repository how-git-works1/.git/
how-git-works1 (main)> git add test.txt

how-git-works1 (main)> ./git-rs read-index
SimpleGit { command: ReadIndex }
index version: 2
entry_count: 1
reading the 0 entry
entry hash: "�\u{1}6%\u{3}\u{b}�۩\u{6}�V�\u{7f}����FJ"
flags: 8
stage: 0
name length: 8
name: test.txt
pad entry: 62
version: 2
hash: �6%
         �۩�V�����FJ
name: test.txt, size: 6
mode: 100644 created_at: 2024-09-01 12:23:24.061000000 modified_at: 2024-09-01 12:23:24.061000000

how-git-works1 (main)> ./git-rs add --path git_obj_0826_6.txt
SimpleGit { command: Add(AddOpts { path_spec: "git_obj_0826_6.txt", all: false }) }
add file options: AddOpts { path_spec: "git_obj_0826_6.txt", all: false }
dot_git: how-git-works1/.git
index version: 2
entry_count: 1
reading the 0 entry
entry hash: "�\u{1}6%\u{3}\u{b}�۩\u{6}�V�\u{7f}����FJ"
flags: 8
stage: 0
name length: 8
name: test.txt
pad entry: 62
Index { version: 2, entries: [Entry { hash: [206, 1, 54, 37, 3, 11, 168, 219, 169, 6, 247, 86, 150, 127, 158, 156, 163, 148, 70, 74], name: "test.txt", created_at: SystemTime { tv_sec: 1725193404, tv_nsec: 61000000 }, modified_at: SystemTime { tv_sec: 1725193404, tv_nsec: 61000000 }, dev: 16777232, inode: 43762421, mode: 33188, uid: 501, gid: 20, size: 6, stage: 0, skip_worktree: false, intent_to_add: false }], cache: Tree { entries: [] } }
file_path: how-git-works1/git_obj_0826_6.txt
hash: [174, 107, 166, 193, 20, 254, 123, 181, 84, 119, 23, 227, 178, 127, 25, 248, 19, 25, 111, 30], len: 20
[write_blob] blob dir: .git/objects/ae
[write_blob] file_name: .git/objects/ae/6ba6c114fe7bb5547717e3b27f19f813196f1e
created file
successfully write object to: .git/objects/ae/6ba6c114fe7bb5547717e3b27f19f813196f1e

set index file back to: how-git-works1/.git/index
encoding index
encoding headers
encoding entries
encoding entry: git_obj_0826_6.txt
[encode_entry] entry: git_obj_0826_6.txt hash: "�k��\u{14}�{�Tw\u{17}�\u{7f}\u{19}�\u{13}\u{19}o\u{1e}", len: 20
encoding entry: test.txt
[encode_entry] entry: test.txt hash: "�\u{1}6%\u{3}\u{b}�۩\u{6}�V�\u{7f}����FJ", len: 20
encoding footer
write footer done
flush into writer
encode done

how-git-works1 (main)> git status
On branch main

No commits yet

Changes to be committed:
  (use "git rm --cached <file>..." to unstage)
	new file:   git_obj_0826_6.txt
	new file:   test.txt

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	git-rs

```

## 24-08-25-01

* use zlib to compress blob bytes

```sh
hash: d53bfed792e3b78e556331e4e2d134b3d4dc4e20
[write_blob] blob dir: .git/objects/d5
[write_blob] file_name: .git/objects/d5/3bfed792e3b78e556331e4e2d134b3d4dc4e20
created file
successfully write object to: .git/objects/d5/3bfed792e3b78e556331e4e2d134b3d4dc4e20
```

* check with git cat-file

```sh
> cat git_obj.txt
0825 add git object

> git cat-file -t d53bfed792e3b78e556331e4e2d134b3d4dc4e20
blob

> git cat-file -p d53bfed
0825 add git object
```

## 24-08-25

* parse index entry

```sh
./git-rs add --path .
SimpleGit { command: Add(AddOpts { path_spec: ".", all: false }) }
add file options: AddOpts { path_spec: ".", all: false }
index version: 2
entry_count: 4
reading the 0 entry
entry hash: "�k�m1]F(\u{b}��TaL�-\u{16}w��"
flags: 11
stage: 0
name length: 11
name: bak/new.txt
pad entry: 62
push entry to index
reading the 1 entry
entry hash: "��Se���\u{3}����\t�\u{c}�,8�"
flags: 12
stage: 0
name length: 12
name: bak/test.txt
pad entry: 62
push entry to index
reading the 2 entry
entry hash: "�k�m1]F(\u{b}��TaL�-\u{16}w��"
flags: 7
stage: 0
name length: 7
name: new.txt
pad entry: 62
push entry to index
reading the 3 entry
entry hash: "�7�Å�:\u{e}\u{1}�9c0�\u{1a}��\u{18}��"
flags: 8
stage: 0
name length: 8
name: test.txt
pad entry: 62
push entry to index
```

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
