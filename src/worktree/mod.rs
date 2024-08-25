use std::fs::File;

use crate::plumbing::index::{decoder, Index};

pub struct Worktree {
    root: String,
}

impl Worktree {
    pub fn new(root: String) -> Self {
        Worktree { root }
    }

    pub fn add(&mut self) -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;
        let path = current_dir.join(".git/index");
        let idx_file: &str = path.to_str().unwrap();
        let index = Index::build(idx_file)?;

        println!("{:?}", index);
        Ok(())
    }
}
