#[derive(Debug)]
pub struct Commit {
    pub tree: Vec<u8>,
    pub parent: Vec<u8>,
    pub author: String,
    pub committer: String,
    pub message: String,
}

impl Commit {
    pub fn new(
        tree: Vec<u8>,
        parent: Vec<u8>,
        author: String,
        committer: String,
        message: String,
    ) -> Self {
        Commit {
            tree,
            parent,
            author,
            committer,
            message,
        }
    }

    pub fn write_commit(&self, commit_path: &str) -> anyhow::Result<()> {
        todo!()
    }
}
