use crate::plumbing::hash::Hash;
use anyhow::Result;
use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};

pub struct DotGit<'a> {
    pub dot_git_path: &'a str,
    pub object_list: Vec<&'a Hash>,
    pub files: HashMap<&'a Hash, File>,
}

impl<'a> DotGit<'a> {
    pub fn new(dot_git_path: &'a str) -> Self {
        DotGit {
            dot_git_path,
            object_list: Vec::new(),
            files: HashMap::new(),
        }
    }

    pub fn initialize(&self) -> Result<()> {
        let must_exists = [
            format!("{}/objects/info", self.dot_git_path),
            format!("{}/objects/pack", self.dot_git_path),
            format!("{}/refs/heads", self.dot_git_path),
            format!("{}/refs/tags", self.dot_git_path),
        ];

        for path in must_exists.iter() {
            if Path::new(path).exists() {
                continue;
            }

            fs::create_dir(path.clone())?;
        }

        Ok(())
    }
}
