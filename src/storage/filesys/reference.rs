use anyhow::Result;

use super::dotgit::DotGit;
use crate::plumbing::reference::Reference;

pub struct ReferenceStore {
    pub dot_git: DotGit,
}

impl ReferenceStore {
    pub fn new(dot_git: DotGit) -> Self {
        ReferenceStore { dot_git }
    }

    pub fn set_ref(&self, reference: &Reference) -> Result<()> {
        println!("[ReferenceStore] set_ref");
        self.dot_git.set_ref(reference, None)
    }
}
