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

        let conf = Config::new(
            Core::new(false, "".to_string(), "#".to_string(), "0".to_string()),
            None,
            branches,
        );

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
}
