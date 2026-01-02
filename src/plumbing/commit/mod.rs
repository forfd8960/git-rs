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

use crate::plumbing::object::Signature;

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