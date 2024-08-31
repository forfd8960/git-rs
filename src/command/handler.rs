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

        GitSubCommand::ReadIndex => {
            let root = current_dir.to_str().unwrap();
            let work_tree = Worktree::new(root.to_string());
            work_tree.read_index()?
        }
        GitSubCommand::Config(opts) => {
            println!("config options: {:?}", opts);
        }
    }
    Ok(())
}
