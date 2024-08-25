use std::{fs::File, io::Read};

use crate::plumbing::{
    hash,
    index::Index,
    object::{self, ObjectType},
};

pub struct Worktree {
    root: String,
}

impl Worktree {
    pub fn new(root: String) -> Self {
        Worktree { root }
    }

    pub fn add(&mut self, add_file: &str) -> anyhow::Result<()> {
        let current_dir = self.root.clone();

        let dot_git = current_dir.clone() + "/.git";
        println!("dot_git: {}", dot_git);

        let index_path = dot_git.clone() + "/index";
        let index = Index::build(&index_path)?;
        println!("{:?}", index);

        let file_path = current_dir.clone() + "/" + add_file;
        println!("file_path: {}", file_path);

        let mut file = File::open(file_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        let hash_str = hash::compute_hash(&ObjectType::BlobObject, content.as_ref());
        println!("hash: {}", hash_str);

        let obj_blob_path = object::write_blob(content, &hash_str)?;

        println!("successfully write object to: {}", obj_blob_path);
        Ok(())
    }
}
