use crate::{
    config::Config,
    errors::GitError,
    plumbing::{
        commit::Commit,
        hash::{compute_hash, Hash},
        object::{self, ObjectType, Signature},
        tree::BuildTreeHelper,
    },
    worktree::Worktree,
};

pub trait Committer {
    // msg string, opts *CommitOptions
    fn commit(&self, msg: &str, opts: &mut CommitOptions) -> Result<Hash, GitError>;
}

/*
// CommitOptions describes how a commit operation should be performed.
type CommitOptions struct {
    // All automatically stage files that have been modified and deleted, but
    // new files you have not told Git about are not affected.
    All bool
    // AllowEmptyCommits enable empty commits to be created. An empty commit
    // is when no changes to the tree were made, but a new commit message is
    // provided. The default behavior is false, which results in ErrEmptyCommit.
    AllowEmptyCommits bool
    // Author is the author's signature of the commit. If Author is empty the
    // Name and Email is read from the config, and time.Now it's used as When.
    Author *object.Signature
    // Committer is the committer's signature of the commit. If Committer is
    // nil the Author signature is used.
    Committer *object.Signature
    // Parents are the parents commits for the new commit, by default when
    // len(Parents) is zero, the hash of HEAD reference is used.
    Parents []plumbing.Hash
    // SignKey denotes a key to sign the commit with. A nil value here means the
    // commit will not be signed. The private key must be present and already
    // decrypted.
    SignKey *openpgp.Entity
    // Signer denotes a cryptographic signer to sign the commit with.
    // A nil value here means the commit will not be signed.
    // Takes precedence over SignKey.
    Signer Signer
    // Amend will create a new commit object and replace the commit that HEAD currently
    // points to. Cannot be used with All nor Parents.
    Amend bool
}
*/
pub struct CommitOptions {
    all: bool,
    allow_empty_commits: bool,
    author: Option<Signature>,
    committer: Option<Signature>,
    parents: Vec<Hash>,
    amend: bool,
    git_path: String,
}

impl CommitOptions {
    pub fn new(git_path: &str) -> Self {
        CommitOptions {
            all: false,
            allow_empty_commits: false,
            author: None,
            committer: None,
            parents: Vec::new(),
            amend: false,
            git_path: git_path.to_string(),
        }
    }

    pub fn validate(&mut self) -> Result<(), GitError> {
        if self.amend && self.all {
            return Err(GitError::InvalidCommitOptions(
                "all and amend cannot be used together".to_string(),
            ));
        }
        if self.amend && !self.parents.is_empty() {
            return Err(GitError::InvalidCommitOptions(
                "parents cannot be used with amend".to_string(),
            ));
        }

        if self.author.is_none() {
            self.load_config_author_and_committer()?;
        }
        if self.committer.is_none() {
            self.committer = self.author.clone();
        }

        if self.parents.is_empty() {
            //load head hash
            let head_hash = object::head_ref(&self.git_path)?;
            self.parents = vec![Hash::from(head_hash.as_str())];
        }

        Ok(())
    }

    fn load_config_author_and_committer(&mut self) -> Result<(), GitError> {
        let mut conf = Config::default();
        conf.git_path = self.git_path.clone();
        conf.load()?;

        let user = conf.get_user()?;
        self.author = Some(Signature::new(user.name.clone(), user.email.clone()));
        self.committer = self.author.clone();
        Ok(())
    }
}

impl Committer for Worktree {
    fn commit(&self, msg: &str, opts: &mut CommitOptions) -> Result<Hash, GitError> {
        println!("validate commit options");
        opts.validate()?;

        if opts.all {
            println!("auto load modified and deleted files");
            self.auto_add_modified_and_deleted()?;
        }

        if opts.amend {
            let head = object::head_ref(&self.git_dir_path)?;
            let head_commit = self.obj_store.read_commit(&head)?;
            opts.parents = head_commit
                .parent_hashes
                .iter()
                .map(|ph| Hash::from(ph.clone()))
                .collect();
        }

        let idx = self.read_index()?;
        if opts.parents.is_empty() && idx.entries.is_empty() && !opts.allow_empty_commits {
            return Err(GitError::EmptyCommit);
        }
        let mut tree_helper = BuildTreeHelper::new(&self.root_path);
        let tree_hash = tree_helper.build_tree(&idx)?;

        let prev_tree = if !opts.parents.is_empty() {
            let parent_commit = self.obj_store.read_commit(&opts.parents[0].to_string())?;
            Hash::from(parent_commit.tree_hash)
        } else {
            Hash::new([0; 20])
        };

        if tree_hash == prev_tree && !opts.allow_empty_commits {
            return Err(GitError::EmptyCommit);
        }

        let commit = Commit::new(
            msg,
            tree_hash.0.to_vec(),
            opts.parents.iter().map(|ph| ph.0.to_vec()).collect(),
            opts.author.clone().unwrap(),
            opts.committer.clone().unwrap(),
        );

        let data = commit.encode();
        let commit_hash = compute_hash(&ObjectType::CommitObject, &data);

        println!(
            "writing commit object to object store: {}",
            Hash::from(commit_hash.clone()).to_string()
        );

        let _ = self.obj_store.write_commit(data, &commit_hash)?;

        println!("update HEAD to point to new commit");
        self.update_head(&commit_hash)?;
        Ok(Hash::from(commit_hash))
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::plumbing::object::Signature;

    #[test]
    fn test_worktree_commit() {
        let author = Signature::new("John Doe".to_string(), "john.doe@example.com".to_string());
        let committer = Signature::new("John Doe".to_string(), "john.doe@example.com".to_string());
        // let commit = Commit::new(
        //     "Initial commit",
        //     vec![0; 20],
        //     vec![],
        //     author.clone(),
        //     committer.clone(),
        // );

        let root = env::var("GIT_TEST_WT_PATH").unwrap_or_else(|_| "/tmp/git_test".to_string());
        println!("root: {}", root);

        let wt = Worktree::new(&root);
        println!("worktree: {:?}", wt);

        let mut opts = CommitOptions::new(&wt.git_dir_path);
        opts.all = true;
        opts.author = Some(author);
        opts.committer = Some(committer);
        let commit_res = wt.commit("Initial commit", &mut opts);
        println!("commit_res: {:?}", commit_res);

        assert!(commit_res.is_ok());
        println!("commit hash: {}", commit_res.unwrap().to_string());
    }
}
