pub mod decoder;
pub mod encoder;

/*
// Stage during merge
type Stage int

const (
    // Merged is the default stage, fully merged
    Merged Stage = 1
    // AncestorMode is the base revision
    AncestorMode Stage = 1
    // OurMode is the first tree revision, ours
    OurMode Stage = 2
    // TheirMode is the second tree revision, theirs
    TheirMode Stage = 3
)
*/

use std::time;

#[derive(Debug, Clone)]
pub enum Stage {
    Merged,       // Merged is the default stage, fully merged
    AncestorMode, // AncestorMode is the base revision
    OurMode,      // OurMode is the first tree revision, ours
    TheirMode,    // TheirMode is the second tree revision, theirs
}

// Entry represents a single file (or stage of a file) in the cache. An entry
// represents exactly one stage of a file. If a file path is unmerged then
// multiple Entry instances may appear for the same path name.
#[derive(Debug, Clone)]
pub struct Entry {
    // Hash is the SHA1 of the represented file
    pub hash: Vec<u8>,
    // Name is the  Entry path name relative to top level directory
    pub name: String,
    // CreatedAt time when the tracked path was created
    pub created_at: time::SystemTime,
    // ModifiedAt time when the tracked path was changed
    pub modified_at: time::SystemTime,
    // Dev and Inode of the tracked path
    pub dev: u32,
    pub inode: u32,
    // Mode of the path
    pub mode: u32,
    // UID and GID, userid and group id of the owner
    pub uid: u32,
    pub gid: u32,
    // Size is the length in bytes for regular files
    pub size: u32,
    // Stage on a merge is defines what stage is representing this entry
    // https://git-scm.com/book/en/v2/Git-Tools-Advanced-Merging
    pub stage: Stage,
    // skip_worktree used in sparse checkouts
    // https://git-scm.com/docs/git-read-tree#_sparse_checkout
    pub skip_worktree: bool,
    // intent_to_add record only the fact that the path will be added later
    pub intent_to_add: bool,
}

// Tree contains pre-computed hashes for trees that can be derived from the
// index. It helps speed up tree object generation from index for a new commit.
// type Tree struct {
// 	Entries []TreeEntry
// }
#[derive(Debug)]
pub struct Tree {
    pub entries: Vec<TreeEntry>,
}

/*
// TreeEntry entry of a cached Tree
*/
#[derive(Debug)]
pub struct TreeEntry {
    pub path: String,
    pub entries: i32,
    pub trees: i32,
    pub hash: Vec<u8>,
}

// Index contains the information about which objects are currently checked out
// in the worktree, having information about the working files. Changes in
// worktree are detected using this Index. The Index is also used during merges
#[derive(Debug)]
pub struct Index {
    pub version: u32,
    pub entries: Vec<Entry>,
    pub cache: Tree,
}

impl Index {
    pub fn new() -> Self {
        Index {
            version: 2,
            entries: Vec::new(),
            cache: Tree {
                entries: Vec::new(),
            },
        }
    }

    pub fn add(&mut self, path: String) -> Entry {
        let e = Entry {
            hash: Vec::new(),
            name: path,
            created_at: time::SystemTime::now(),
            modified_at: time::SystemTime::now(),
            dev: 0,
            inode: 0,
            mode: 0,
            uid: 0,
            gid: 0,
            size: 0,
            stage: Stage::Merged,
            skip_worktree: false,
            intent_to_add: false,
        };

        self.entries.push(e.clone());
        e
    }
}
