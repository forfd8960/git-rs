use clap::Parser;
use git_rs::command::SimpleGit;

fn main() {
    let cmd = SimpleGit::parse();
    println!("{:?}", cmd);
}
