use std::collections::HashMap;

use crate::{
    config::{Config, Core},
    plumbing::{MAIN, MAIN_REF},
    storage::{filesystem::FileSystem, Store},
};

const GIT_DIR: &str = ".git";

pub struct Repository {
    config: Config,
    wt_path: String,
    storer: FileSystem,
}

impl Repository {
    pub fn new(path: &str) -> Self {
        let branches = HashMap::from([(
            MAIN.to_string(),
            crate::config::Branch {
                name: MAIN.to_string(),
                remote: "".to_string(),
                merge: MAIN_REF.to_string(),
                rebase: "".to_string(),
                description: "".to_string(),
            },
        )]);

        let conf = Config::default();

        Repository {
            config: conf,
            wt_path: path.to_string(),
            storer: FileSystem::new(path),
        }
    }

    pub fn init(&self) -> anyhow::Result<()> {
        self.storer.init_dot_git()?;
        self.config
            .init(&format!("{}/{}/config", self.wt_path, GIT_DIR))?;
        Ok(())
    }

    pub fn head(&self) -> anyhow::Result<String> {
        let head_path = format!("{}/{}/HEAD", self.wt_path, GIT_DIR);
        let head_content = std::fs::read_to_string(head_path)?;
        Ok(head_content.trim().to_string())
    }
}
