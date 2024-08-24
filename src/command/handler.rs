use std::env;

use crate::repo::Repository;

use super::GitSubCommand;

pub fn handle_command(cmd: &GitSubCommand) -> anyhow::Result<()> {
    match cmd {
        GitSubCommand::Init(opts) => {
            println!("init repo optiond: {:?}", opts);
            let current_dir = env::current_dir().unwrap();
            let repo = Repository::new(current_dir.to_str().unwrap());
            repo.init()?;
        }
        GitSubCommand::Add(opts) => {
            todo!()
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
    }
    Ok(())
}
