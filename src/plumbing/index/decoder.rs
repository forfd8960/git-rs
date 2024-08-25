use anyhow::bail;
use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;
use std::time::SystemTime;

use crate::errors::GitError;
use crate::plumbing::hash;

use super::Entry;
use super::Index;

const INDEX_SIG: [u8; 4] = [b'D', b'I', b'R', b'C'];
// DecodeVersionSupported = struct{ Min, Max uint32 }{Min: 2, Max: 4}
const INDEX_VERSION_MIN: u32 = 2;
const INDEX_VERSION_MAX: u32 = 4;

/*
const (
    entryHeaderLength = 62
    entryExtended     = 0x4000
    entryValid        = 0x8000
    nameMask          = 0xfff
    intentToAddMask   = 1 << 13
    skipWorkTreeMask  = 1 << 14
)
*/
const ENTRY_HEADER_LENGTH: u32 = 62;
const ENTRY_EXTENDED: u16 = 0x4000;
const ENTRY_VALID: u32 = 0x8000;
const ENTRY_NAME_MASK: u16 = 0xfff;
const INTENT_TO_ADD_MASK: u16 = 1 << 13;
const SKIP_WORKTREE_MASK: u16 = 1 << 14;

pub struct Decoder {
    pub index_file: File,
}

impl Decoder {
    pub fn new(idx_file: File) -> Self {
        Decoder {
            index_file: idx_file,
        }
    }

    pub fn decode(&mut self, idx: &mut Index) -> Result<()> {
        idx.version = self.validate_header()?;
        println!("index version: {}", idx.version);

        let entry_count = self.read_u32()?;
        println!("entry_count: {}", entry_count);

        self.read_entries(entry_count, idx)?;

        Ok(())
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(self.index_file.read_u32::<BigEndian>()?)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        Ok(self.index_file.read_u16::<BigEndian>()?)
    }

    pub fn read_hash(&mut self) -> Result<Vec<u8>> {
        let mut buf = [0; hash::SIZE as usize];
        self.index_file.read_exact(&mut buf)?;
        Ok(buf.to_vec())
    }

    fn validate_header(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.index_file.read_exact(&mut buf)?;

        if !buf.eq(&INDEX_SIG) {
            bail!(GitError::MalformedIndexSignature);
        }

        let version = self.read_u32()?;
        if version < INDEX_VERSION_MIN || version > INDEX_VERSION_MAX {
            bail!(GitError::UnsupportedIndexVersion);
        }
        Ok(version)
    }

    fn read_entries(&mut self, entry_count: u32, idx: &mut Index) -> Result<()> {
        for entry_idx in 0..entry_count {
            println!("reading the {} entry", entry_idx);

            self.read_entry(idx)?;
        }
        Ok(())
    }

    fn read_entry(&mut self, idx: &mut Index) -> Result<()> {
        let mut entry = Entry::new();
        let sec = self.read_u32()?;
        let nsec = self.read_u32()?;
        let msec = self.read_u32()?;
        let mnsec = self.read_u32()?;

        let tm = SystemTime::UNIX_EPOCH
            + Duration::from_secs(sec as u64)
            + Duration::from_millis(nsec as u64);

        entry.created_at = tm;
        entry.modified_at = SystemTime::UNIX_EPOCH
            + Duration::from_secs(msec as u64)
            + Duration::from_millis(mnsec as u64);

        entry.dev = self.read_u32()?;
        entry.inode = self.read_u32()?;
        entry.mode = self.read_u32()?;
        entry.uid = self.read_u32()?;
        entry.gid = self.read_u32()?;
        entry.size = self.read_u32()?;
        entry.hash = self.read_hash()?;

        println!("entry hash: {:?}", String::from_utf8_lossy(&entry.hash));

        let flags = self.read_u16()?;
        println!("flags: {}", flags);

        entry.stage = (flags >> 12) & 0x3;
        println!("stage: {}", entry.stage);

        let mut read_len = ENTRY_HEADER_LENGTH;

        if flags & ENTRY_EXTENDED != 0 {
            println!("entry extend");

            let extended = self.read_u16()?;
            read_len += 2;

            entry.intent_to_add = extended & INTENT_TO_ADD_MASK != 0;
            entry.skip_worktree = extended & SKIP_WORKTREE_MASK != 0;
        }

        entry.name = self.read_entry_name(idx, flags)?;
        println!("name: {}", entry.name);
        self.pad_entry(&entry, read_len)?;

        println!("push entry to index");
        idx.entries.push(entry);

        Ok(())
    }

    fn read_entry_name(&mut self, idx: &Index, flags: u16) -> Result<String> {
        match idx.version {
            2 | 3 => {
                let len = (flags & ENTRY_NAME_MASK) as usize;
                println!("name length: {}", len);

                let mut buf = vec![0; len];
                self.index_file.read_exact(&mut buf)?;
                Ok(String::from_utf8_lossy(&buf).to_string())
            }
            _ => bail!(GitError::NotSupportedIndexVersion),
        }
    }

    fn pad_entry(&mut self, e: &Entry, read_len: u32) -> Result<()> {
        println!("pad entry: {}", read_len);

        let entry_size = read_len + e.name.len() as u32;

        let pad_len = 8 - entry_size % 8;
        self.index_file.seek(SeekFrom::Current(pad_len as i64))?;
        Ok(())
    }
}
