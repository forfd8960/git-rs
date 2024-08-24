use clap::{Parser, Subcommand};

pub mod handler;

#[derive(Debug, Parser)]
#[command(name="simple-git", version="0.0.1", about, long_about = None)]
pub struct SimpleGit {
    #[command(subcommand)]
    pub command: GitSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum GitSubCommand {
    #[command(name = "init", about = "init a git repo")]
    Init(InitOpts),
    #[command(name = "add", about = "add files into index")]
    Add(AddOpts),
    #[command(
        name = "commit",
        about = "Create a new commit containing the current contents of the index and the given log message describing the changes"
    )]
    Commit(CommitOpts),
    #[command(name = "log", about = "show commit logs")]
    Log(LogOpts),
    #[command(
        name = "branch",
        about = "git-branch - List, create, or delete branches"
    )]
    Branch(BranchOpts),
}

#[derive(Debug, Parser)]
pub struct InitOpts {
    /// Only print error and warning messages; all other output will be suppressed.
    #[arg(short, long)]
    pub quiet: bool,

    /// Specify the given object <format> (hash algorithm) for the repository. The valid values are sha1 and (if enabled) sha256. sha1 is the default.
    #[arg(long = "object-format", default_value = "sha1")]
    pub object_format: Option<String>,

    #[arg(long = "ref-format", default_value = "files")]
    pub ref_format: Option<String>,

    /// Specify the directory from which templates will be used.
    #[arg(long = "template")]
    pub template: Option<String>,

    /// Use <branch-name> for the initial branch in the newly created repository. If not specified, fall back to the default name.
    #[arg(short, long = "initial-branch")]
    pub branch: Option<String>,
}

#[derive(Debug, Parser)]
pub struct AddOpts {
    /// Files to add content from
    #[arg(short, long = "path")]
    pub path_spec: String,

    /// If no <pathspec> is given when -A option is used, all files in the entire working tree are updated
    #[arg(short, long = "all")]
    pub all: bool,
}

#[derive(Debug, Parser)]
pub struct CommitOpts {
    /// Tell the command to automatically stage files that have been modified and deleted, but new files you have not told Git about are not affected.
    #[arg(short, long)]
    pub all: bool,

    /// Record changes to the repository
    #[arg(short, long)]
    pub message: String,

    /// The message taken from file with -F, command line with -m, and from commit object with -C are usually used as the commit log message unmodified. This option lets you further edit the message taken from these sources.
    #[arg(short, long = "edit")]
    pub edit: bool,

    /// Use the selected commit message without launching an editor
    #[arg(short, long = "no-edit")]
    pub no_edit: bool,

    /// Replace the tip of the current branch by creating a new commit.
    #[arg(long)]
    pub amend: bool,
}

#[derive(Debug, Parser)]
pub struct LogOpts {
    /// This is a shorthand for "--pretty=oneline --abbrev-commit" used together.
    #[arg(short, long = "oneline")]
    pub oneline: bool,

    #[arg(short, long)]
    pub number: i32,

    /// Limit the number of commits to output.
    #[arg(long = "max-count")]
    pub max_count: i32,

    /// Skip number commits before starting to show the commit output.
    #[arg(long = "skip")]
    pub skip: i32,

    /// Show commits more recent than a specific date.
    #[arg(long = "since")]
    pub since: String,
    /// Show commits more recent than a specific date.
    #[arg(long = "after")]
    pub after: String,
    /// Show commits older than a specific date.
    #[arg(long = "until")]
    pub until: String,
    /// Show commits older than a specific date.
    #[arg(long = "before")]
    pub before: String,

    /// --author=<pattern>, --committer=<pattern>
    #[arg(long = "author")]
    pub author: String,
    /// commiter
    #[arg(long = "committer")]
    pub committer: String,

    /// Pretty-print the contents of the commit logs in a given format, where <format> can be one of oneline,
    #[arg(long = "pretty")]
    pub pretty: String,
    /// Pretty-print the contents of the commit logs in a given format, where <format> can be one of oneline,
    #[arg(long = "format")]
    pub format: String,
}

#[derive(Debug, Parser)]
pub struct BranchOpts {
    /// delete a branch
    #[arg(short, long)]
    pub delete: String,

    /// Move/rename a branch, together with its config and reflog.
    #[arg(short = 'm', long = "move")]
    pub move_branch: String,

    /// Copy a branch, together with its config and reflog.
    #[arg(short, long)]
    pub copy: String,

    /// List branches. With optional <pattern>...,
    /// e.g. git branch --list 'maint-*', list only the branches that match the pattern(s).
    #[arg(short, long)]
    pub list: String,
}
