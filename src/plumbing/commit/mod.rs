/*
const (
    beginpgp       string = "-----BEGIN PGP SIGNATURE-----"
    endpgp         string = "-----END PGP SIGNATURE-----"
    headerpgp      string = "gpgsig"
    headerencoding string = "encoding"

    // https://github.com/git/git/blob/bcb6cae2966cc407ca1afc77413b3ef11103c175/Documentation/gitformat-signature.txt#L153
    // When a merge commit is created from a signed tag, the tag is embedded in
    // the commit with the "mergetag" header.
    headermergetag string = "mergetag"

    defaultUtf8CommitMessageEncoding MessageEncoding = "UTF-8"
)

// Hash represents the hash of an object
type Hash plumbing.Hash

// MessageEncoding represents the encoding of a commit
type MessageEncoding string

// Commit points to a single tree, marking it as what the project looked like
// at a certain point in time. It contains meta-information about that point
// in time, such as a timestamp, the author of the changes since the last
// commit, a pointer to the previous commit(s), etc.
// http://shafiulazam.com/gitbook/1_the_git_object_model.html
type Commit struct {
    // Hash of the commit object.
    Hash plumbing.Hash
    // Author is the original author of the commit.
    Author Signature
    // Committer is the one performing the commit, might be different from
    // Author.
    Committer Signature
    // MergeTag is the embedded tag object when a merge commit is created by
    // merging a signed tag.
    MergeTag string
    // PGPSignature is the PGP signature of the commit.
    PGPSignature string
    // Message is the commit message, contains arbitrary text.
    Message string
    // TreeHash is the hash of the root tree of the commit.
    TreeHash plumbing.Hash
    // ParentHashes are the hashes of the parent commits of the commit.
    ParentHashes []plumbing.Hash
    // Encoding is the encoding of the commit.
    Encoding MessageEncoding
    // List of extra headers of the commit
    ExtraHeaders []ExtraHeader

    s storer.EncodedObjectStorer
}

// ExtraHeader holds any non-standard header
type ExtraHeader struct {
    // Header name
    Key string
    // Value of the header
    Value string
}
*/
use std::fmt::Write;
use chrono::{DateTime, FixedOffset};

use crate::{plumbing::object::{self, Signature}};

type MessageEncoding = String;

#[derive(Debug, Clone)]
pub struct ExtraHeader {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Commit {
    // Hash of the commit object.
    pub hash: Vec<u8>,
    // Author is the original author of the commit.
    pub author: Signature,
    // Committer is the one performing the commit, might be different from
    // Author.
    pub committer: Signature,
    // MergeTag is the embedded tag object when a merge commit is created by
    // merging a signed tag.
    pub merge_tag: String,
    // PGPSignature is the PGP signature of the commit.
    pub pgp_signature: String,
    // Message is the commit message, contains arbitrary text.
    pub message: String,
    // TreeHash is the hash of the root tree of the commit.
    pub tree_hash: Vec<u8>,
    // ParentHashes are the hashes of the parent commits of the commit.
    pub parent_hashes: Vec<Vec<u8>>,
    // Encoding is the encoding of the commit.
    pub encoding: MessageEncoding,
    // List of extra headers of the commit
    pub extra_headers: Vec<ExtraHeader>,
}


impl Commit {
    // commit <size>\0<content>
    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        let tree_hash = base16ct::lower::encode_string(&self.tree_hash);
        let tree_hash = format!(
            "tree {}\n",
            tree_hash
        );
        encoded.extend_from_slice(tree_hash.as_bytes());

        for parent_hash in &self.parent_hashes {
            let parent_hash_str = base16ct::lower::encode_string(parent_hash);
            let parent_line = format!("parent {}\n", parent_hash_str);
            encoded.extend_from_slice(parent_line.as_bytes());
        }

        encoded.extend_from_slice(
            &self.encode_author(),
        );
        encoded.extend_from_slice(
            &self.encode_committer()
        );
        encoded.extend_from_slice(b"\n\n");
        encoded.extend_from_slice(self.message.as_bytes());

        let obj_header = format!(
            "{} {}\0",
            object::OBJ_COMMIT_HEADER,
            encoded.len()
        );

        let new_encoded = [&obj_header.as_bytes()[..], &encoded[..]].concat();
        new_encoded
    }

    pub fn encode_author(&self) -> Vec<u8> {
        self.author.encode().as_bytes().to_vec()
    }

    pub fn encode_committer(&self) -> Vec<u8> {
        self.committer.encode().as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::plumbing::hash::Hash;

    use super::*;
    use chrono::offset::TimeZone;

    #[test]
    fn test_commit_encode() {
        let author = Signature {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            when: FixedOffset::east_opt(0)
                .unwrap()
                .with_ymd_and_hms(2026, 1, 1, 12, 0, 0)
                .unwrap(),
        };
        let committer = author.clone();
        let commit = Commit {
            hash: vec![],
            author,
            committer,
            merge_tag: "".to_string(),
            pgp_signature: "".to_string(),
            message: "Initial commit".to_string(),
            tree_hash: Hash::from("4b825dc642cb6eb9a060e54bf8d69288fbee4904").0.to_vec(),
            parent_hashes: vec![],
            encoding: "UTF-8".to_string(),
            extra_headers: vec![],
        };
        let encoded = commit.encode();
        println!("committed obj data: {}\n", String::from_utf8_lossy(&encoded));

        let encoded_str = String::from_utf8_lossy(&encoded);
        let expect_commit =r#"commit 177\0tree 4b825dc642cb6eb9a060e54bf8d69288fbee4904
author John Doe <john.doe@example.com> 1767268800 +0000
committer John Doe <john.doe@example.com> 1767268800 +0000


Initial commit"#;
        assert_eq!(encoded_str, expect_commit);
    }
}