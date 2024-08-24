use anyhow::Result;
use clap::Parser;
use git_rs::command::{handler::handle_command, SimpleGit};

fn main() -> Result<()> {
    let cmd = SimpleGit::parse();
    println!("{:?}", cmd);
    handle_command(&cmd.command)?;
    Ok(())
}
