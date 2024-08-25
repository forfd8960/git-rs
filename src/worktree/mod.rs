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
        let mut idx = Index::new();
        let index_file_path = self.root.clone() + "/.git/index";

        let index_reader = File::open(index_file_path)?;
        let mut index_decoder = decoder::Decoder::new(index_reader);
        index_decoder.decode(&mut idx)?;

        println!("{:?}", idx);
        Ok(())
    }
}
