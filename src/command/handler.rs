use std::env;

use crate::{repo::Repository, worktree::Worktree};

use super::GitSubCommand;

pub fn handle_command(cmd: &GitSubCommand) -> anyhow::Result<()> {
    match cmd {
        GitSubCommand::Init(opts) => {
            println!("init repo options: {:?}", opts);
            let current_dir = env::current_dir().unwrap();
            let repo = Repository::new(current_dir.to_str().unwrap());
            repo.init()?;
        }
        GitSubCommand::Add(opts) => {
            println!("add file options: {:?}", opts);
            let current_dir = env::current_dir().unwrap();
            let root = current_dir.to_str().unwrap();

            let mut work_tree = Worktree::new(root.to_string());
            work_tree.add(&opts.path_spec)?
        }
        GitSubCommand::Commit(opts) => {
            todo!()
        }
        GitSubCommand::Log(opts) => {
            todo!()
        }
        GitSubCommand::Branch(opts) => {
            todo!()
        }
        GitSubCommand::UpdateIndex(opts) => {
            todo!()
        }
    }
    Ok(())
}
