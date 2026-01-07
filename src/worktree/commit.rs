use crate::{
    config::Config,
    errors::GitError,
    plumbing::{
        hash::Hash,
        object::{self, Signature},
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
        opts.validate()?;

        Ok(Hash([0; 20])) // TODO: implement commit logic
    }
}
