use crate::repo::Repository;

use super::GitSubCommand;

pub fn handle_command(cmd: &GitSubCommand) -> anyhow::Result<()> {
    match cmd {
        GitSubCommand::Init(opts) => {
            println!("init repo optiond: {:?}", opts);
            let repo = Repository::new(".");
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
