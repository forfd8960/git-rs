use std::{collections::HashMap, env};

use crate::{
    config::{Config, Core, User},
    plumbing::MAIN,
    repo::Repository,
    worktree::Worktree,
};

use super::GitSubCommand;

pub fn handle_command(cmd: &GitSubCommand) -> anyhow::Result<()> {
    let current_dir = env::current_dir().unwrap();

    let mut worktree = Worktree::new(current_dir.to_str().unwrap());

    match cmd {
        GitSubCommand::Init(opts) => {
            println!("init repo options: {:?}", opts);
            let repo = Repository::new(current_dir.to_str().unwrap());
            repo.init()?;
        }
        GitSubCommand::Add(opts) => {
            println!("add file options: {:?}", opts);
            worktree.add(&opts.path_spec)?
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

        GitSubCommand::ReadIndex => {
            let idx = worktree.read_index()?;
            println!("{:?}", idx);
        }
        GitSubCommand::Config(opts) => {
            println!("config options: {:?}", opts);
        }
    }
    Ok(())
}
