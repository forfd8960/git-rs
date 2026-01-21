Git uses `.gitignore` files to determine which files and directories it should intentionally *not* track. When you run Git commands like `git status` or `git add`, Git consults these patterns to decide what to show as untracked or what to exclude from being added to the staging area.

### Sources of Ignore Patterns

Git checks for ignore patterns from several sources, with a specific order of precedence [1](https://git-scm.com/docs/gitignore):

1.  **Command-line patterns**: Patterns provided directly to Git commands (less common for general ignoring).
2.  **`.gitignore` files**: Located in the same directory as the path being checked, or in any parent directory up to the repository root. Patterns in `.gitignore` files closer to the file (i.e., in subdirectories) override patterns in higher-level `.gitignore` files. These are typically version-controlled.
3.  **`$GIT_DIR/info/exclude`**: A file specific to a particular repository but not shared with others. Useful for personal ignore rules within a project.
4.  **`core.excludesFile`**: A global ignore file specified in your Git configuration (e.g., `~/.gitconfig`). This file contains patterns that Git should ignore across *all* your repositories (e.g., editor backup files).

When deciding whether to ignore a path, Git applies all relevant patterns from these sources. The **last matching pattern** determines the outcome [1](https://git-scm.com/docs/gitignore).

### Pattern Matching Rules

Git's `.gitignore` patterns use a glob-style syntax, similar to shell wildcards, but with some Git-specific extensions [1](https://git-scm.com/docs/gitignore):

1.  **Blank lines**: Ignored, can be used for readability.
2.  **Comments**: Lines starting with `#` are comments. If a pattern itself starts with `#`, escape it with a backslash (`\#`) [1](https://git-scm.com/docs/gitignore).
3.  **Trailing spaces**: Ignored unless escaped with a backslash (`\ `) [1](https://git-scm.com/docs/gitignore).
4.  **`*` (Asterisk)**: Matches anything except a slash (`/`).
    *   `*.log` matches `debug.log`, `error.log`.
    *   `build/*` matches `build/temp.o`, `build/output.exe`.
5.  **`?` (Question Mark)**: Matches any one character except a slash (`/`) [1](https://git-scm.com/docs/gitignore).
    *   `file?.txt` matches `file1.txt`, `fileA.txt`.
6.  **`[]` (Character Range)**: Matches one of the characters in a range [1](https://git-scm.com/docs/gitignore).
    *   `[abc].txt` matches `a.txt`, `b.txt`, `c.txt`.
    *   `[0-9].tmp` matches `1.tmp`, `5.tmp`.
7.  **`/` (Slash as Directory Separator)**:
    *   **Leading slash (`/`)**: Anchors the pattern to the directory where the `.gitignore` file itself resides.
        *   `/temp` matches `temp` in the same directory as `.gitignore`, but not `subdir/temp`.
        *   `temp` (without leading slash) matches `temp` anywhere in the repository, including `subdir/temp`.
    *   **Trailing slash (`/`)**: Indicates that the pattern only matches directories.
        *   `build/` matches the directory `build` and everything inside it.
        *   `doc/frotz/` matches the directory `doc/frotz` but not `a/doc/frotz` [1](https://git-scm.com/docs/gitignore).
    *   **No slash**: If a pattern does not contain a slash, it matches files or directories at any level below the `.gitignore` file's location [1](https://git-scm.com/docs/gitignore).
8.  **`!` (Negation)**: An optional prefix `!` negates the pattern. Any file or directory matching a negated pattern will *not* be ignored, even if a previous pattern would have ignored it [1](https://git-scm.com/docs/gitignore).
    *   `*` (ignore everything)
    *   `!README.md` (re-include `README.md`)
    *   If a pattern starts with `!`, escape it with `\!` if you want to match a literal `!` [1](https://git-scm.com/docs/gitignore).
9.  **`**` (Double Asterisk)**:
    *   **`**/`**: A leading `**/` matches in all directories.
        *   `**/foo` matches `foo` in the root, `dir/foo`, `dir/subdir/foo`.
    *   **`/**`**: A trailing `/**` matches everything inside a directory, recursively.
        *   `abc/**` matches all files and subdirectories inside `abc` [1](https://git-scm.com/docs/gitignore].
    *   **`a/**/b`**: Matches `a/b`, `a/x/b`, `a/x/y/b`, etc.
10. **Important Rule: Parent Directory Exclusion**: It is **not possible to re-include a file if a parent directory of that file is excluded** [1](https://git-scm.com/docs/gitignore). Git doesn't list excluded directories for performance reasons, so any patterns on contained files have no effect.
    *   If you have `build/` in your `.gitignore`, then `!build/main.js` will *not* re-include `build/main.js` because the `build` directory itself is ignored. To re-include files within an ignored directory, you must first re-include the directory itself (or its parent), then the specific files.
    *   Example:
        ```
        # Ignore the entire 'build' directory
        build/

        # This will NOT work to re-include main.js
        !build/main.js
        ```
    *   Correct way to re-include specific files in an otherwise ignored directory:
        ```
        # Ignore the entire 'build' directory
        build/

        # Re-include the 'build' directory itself (or at least allow Git to traverse it)
        !build/

        # Now, re-include specific files within 'build'
        !build/main.js
        ```
        Or, more commonly, if you want to ignore *most* files in `build` but keep a few:
        ```
        # Ignore everything in build
        build/*

        # Re-include the specific file
        !build/main.js
        ```
        The Stack Overflow search result provides a good example of this, explaining that `*` ignores everything, then `!images/` re-includes the `images` directory, allowing `!images/*.*` to then include files within it [2](https://stackoverflow.com/questions/33189437/explain-gitignore-pattern-matching).

### How Git Matches

When Git needs to decide if a file should be ignored, it takes the file's path relative to the repository root and compares it against all applicable patterns from the various `.gitignore` sources. It processes these patterns in order of precedence (as listed above), and within a single `.gitignore` file, patterns are processed line by line. The **last pattern that matches** the file's path determines whether it's ignored or not [1](https://git-scm.com/docs/gitignore).

### Example

Consider this directory structure:

```
my_project/
├── .gitignore
├── README.md
├── src/
│   └── main.py
├── build/
│   ├── output.exe
│   └── temp.o
├── logs/
│   ├── debug.log
│   └── error.log
└── docs/
    ├── user_guide.md
    └── images/
        └── logo.png
```

And this `.gitignore` file:

```
# Ignore all log files
*.log

# Ignore the entire build directory
build/

# But don't ignore the user guide in docs
!docs/user_guide.md

# Ignore all images in the docs directory
docs/images/
```

Here's how Git would match:

*   `README.md`: No match, tracked.
*   `src/main.py`: No match, tracked.
*   `build/output.exe`: Matches `build/`, ignored.
*   `build/temp.o`: Matches `build/`, ignored.
*   `logs/debug.log`: Matches `*.log`, ignored.
*   `logs/error.log`: Matches `*.log`, ignored.
*   `docs/user_guide.md`: Matches `!docs/user_guide.md`. The `!docs/user_guide.md` pattern overrides any potential earlier ignore (though none exist here), so it is **tracked**.
*   `docs/images/logo.png`: Matches `docs/images/`, ignored.

### Testing `.gitignore` Patterns

You can use the `git check-ignore` command to test how Git will interpret specific paths against your ignore rules:

```bash
# Check if a specific file is ignored
git check-ignore -v logs/debug.log

# Check multiple files
git check-ignore -v build/output.exe docs/user_guide.md
```

The `-v` (verbose) flag shows you which pattern in which `.gitignore` file caused the file to be ignored (or not).