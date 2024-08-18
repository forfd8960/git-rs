# Change Log

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
