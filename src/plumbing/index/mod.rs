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

use std::{fmt::Display, fs::File, time};

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use decoder::Decoder;
use encoder::Encoder;

use crate::errors::GitError;

const INDEX_SIG: [u8; 4] = [b'D', b'I', b'R', b'C'];
const INDEX_VERSION_MIN: u32 = 2;
const INDEX_VERSION_MAX: u32 = 4;

const ENTRY_HEADER_LENGTH: u32 = 62;
const ENTRY_EXTENDED: u16 = 0x4000;
const ENTRY_VALID: u32 = 0x8000;
const ENTRY_NAME_MASK: u16 = 0xfff;
const INTENT_TO_ADD_MASK: u16 = 1 << 13;
const SKIP_WORKTREE_MASK: u16 = 1 << 14;

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
    pub stage: u16,
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

    pub fn from(index_path: &str) -> Result<Self> {
        let mut idx = Index::new();

        let index_reader = File::open(index_path)?;
        let mut index_decoder = Decoder::new(index_reader);
        index_decoder.decode(&mut idx)?;
        Ok(idx)
    }

    pub fn set(mut idx: Index, file: File) -> Result<()> {
        let mut idx_encoder: Encoder = Encoder::new(file);
        idx_encoder.encode(&mut idx)?;
        Ok(())
    }

    pub fn entry(&self, path: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.name == path)
    }

    pub fn update_entry(&mut self, new_entry: Entry) -> anyhow::Result<()> {
        let idx = self
            .entries
            .iter_mut()
            .position(|e| e.name == new_entry.name);
        match idx {
            Some(idx) => {
                let old_entry = self.entries.remove(idx);
                self.entries[idx] = Entry::from_old_new_entry(old_entry, new_entry);
            }
            _ => bail!(GitError::EntryNotFound),
        }

        Ok(())
    }

    pub fn add(&mut self, e: &Entry) {
        self.entries.push(e.clone());
    }
}

impl Entry {
    pub fn new() -> Self {
        Entry {
            hash: Vec::new(),
            name: "".to_string(),
            created_at: time::SystemTime::now(),
            modified_at: time::SystemTime::now(),
            dev: 0,
            inode: 0,
            mode: 0,
            uid: 0,
            gid: 0,
            size: 0,
            stage: 0,
            skip_worktree: false,
            intent_to_add: false,
        }
    }

    pub fn from_old_new_entry(old_entry: Entry, new_entry: Entry) -> Self {
        Entry {
            hash: new_entry.hash,
            name: old_entry.name,
            created_at: old_entry.created_at,
            modified_at: new_entry.modified_at,
            dev: new_entry.dev,
            inode: new_entry.inode,
            mode: new_entry.mode,
            uid: new_entry.uid,
            gid: new_entry.gid,
            size: new_entry.size,
            stage: new_entry.stage,
            skip_worktree: old_entry.skip_worktree,
            intent_to_add: old_entry.intent_to_add,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "hash: {}\n",
            String::from_utf8_lossy(self.hash.as_slice())
        )?;
        write!(f, "name: {}, ", self.name)?;
        write!(f, "size: {}\n", self.size)?;
        write!(f, "mode: {:06o} ", self.mode)?;

        //  // Convert SystemTime to DateTime<Utc>
        let create_utc: DateTime<Utc> = self.created_at.into();
        write!(
            f,
            "created_at: {} ",
            create_utc.format("%Y-%m-%d %H:%M:%S%.9f")
        )?;

        let modified_utc: DateTime<Utc> = self.modified_at.into();
        write!(
            f,
            "modified_at: {}\n",
            modified_utc.format("%Y-%m-%d %H:%M:%S%.9f")
        )?;

        Ok(())
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "version: {} \n", self.version)?;
        for e in &self.entries {
            write!(f, "{}", e)?;
        }

        Ok(())
    }
}
