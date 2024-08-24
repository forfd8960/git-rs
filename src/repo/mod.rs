use crate::storage::{filesystem::FileSystem, Store};

pub struct Repository {
    storer: FileSystem,
}

impl Repository {
    pub fn new(path: &str) -> Self {
        Repository {
            storer: FileSystem::new(path),
        }
    }

    pub fn init(&self) -> anyhow::Result<()> {
        self.storer.init_dot_git()?;
        Ok(())
    }
}
