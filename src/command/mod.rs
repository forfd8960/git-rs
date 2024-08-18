use clap::{Parser, Subcommand};

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
}

#[derive(Debug, Parser)]
pub struct InitOpts {
    /// Only print error and warning messages; all other output will be suppressed.
    #[arg(short, long)]
    pub quiet: bool,

    /// Specify the given object <format> (hash algorithm) for the repository. The valid values are sha1 and (if enabled) sha256. sha1 is the default.
    #[arg(long = "object-format", default_value = "sha1")]
    pub object_format: String,

    #[arg(long = "ref-format", default_value = "files")]
    pub ref_format: String,

    /// Specify the directory from which templates will be used.
    #[arg(long = "template")]
    pub template: String,

    /// Use <branch-name> for the initial branch in the newly created repository. If not specified, fall back to the default name.
    #[arg(short, long = "initial-branch")]
    pub branch: String,
}
