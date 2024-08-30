use crate::{
    config::Config,
    storage::{filesystem::FileSystem, Store},
};

const GIT_DIR: &str = ".git";

pub struct Repository {
    config: Config,
    wt_path: String,
    storer: FileSystem,
}

impl Repository {
    pub fn new(path: &str, conf: Config) -> Self {
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
