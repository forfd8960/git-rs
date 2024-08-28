use crate::plumbing::{
    hash::Hash,
    reference::{Reference, ReferenceType},
};
use anyhow::Result;
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

#[derive(Debug)]
pub struct DotGit {
    pub dot_git_path: String,
    pub object_list: Vec<Hash>,
    pub files: HashMap<Hash, File>,
}

impl DotGit {
    pub fn new(dot_git_path: String) -> Self {
        DotGit {
            dot_git_path,
            object_list: Vec::new(),
            files: HashMap::new(),
        }
    }

    fn create_file(&self, path: &str, truncate: bool, content: &str) -> Result<()> {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(truncate)
            .open(path)?;

        f.write_all(content.as_bytes())?;
        Ok(())
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

            fs::create_dir_all(path.clone())?;
        }

        Ok(())
    }

    //(todo): check old and truncate ref file
    pub fn set_ref(&self, r: &Reference, old: Option<Reference>) -> Result<()> {
        let content = match r.ref_type {
            ReferenceType::SymbolicReference => format!("ref: {}\n", r.target.as_ref().unwrap()),
            ReferenceType::HashReference => format!("{}\n", r.hash.as_ref().unwrap().to_string()),
            _ => "".to_string(),
        };

        let path = format!("{}/{}", self.dot_git_path, r.name.0);
        self.create_file(&path, false, &content)?;
        Ok(())
    }
}
