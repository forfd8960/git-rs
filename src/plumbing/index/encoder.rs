use byteorder::{BigEndian, WriteBytesExt};
use std::{
    fs::File,
    io::{BufWriter, Write},
    iter,
    time::UNIX_EPOCH,
};

use super::{Entry, Index, ENTRY_HEADER_LENGTH, ENTRY_NAME_MASK, INDEX_SIG};

pub struct Encoder {
    writer: BufWriter<File>,
}

impl Encoder {
    pub fn new(file: File) -> Self {
        Encoder {
            writer: BufWriter::new(file),
        }
    }

    pub fn encode(&mut self, index: &mut Index) -> anyhow::Result<()> {
        self.encode_header(index)?;
        self.encode_entries(index)?;
        Ok(())
    }

    fn encode_header(&mut self, index: &mut Index) -> anyhow::Result<()> {
        self.writer.write_all(&INDEX_SIG)?;
        self.writer.write_u32::<BigEndian>(index.version)?;
        self.writer
            .write_u32::<BigEndian>(index.entries.len() as u32)?;
        Ok(())
    }

    fn encode_entries(&mut self, index: &mut Index) -> anyhow::Result<()> {
        index.entries.sort_by(|a, b| a.name.cmp(&b.name));
        for entry in &index.entries {
            self.encode_entry(entry)?;

            let mut entry_len = ENTRY_HEADER_LENGTH;
            if entry.intent_to_add || entry.skip_worktree {
                entry_len += 2
            }

            let wrote = entry_len + entry.name.len() as u32;
            self.pad_entry(wrote)?;
        }
        Ok(())
    }

    fn encode_entry(&mut self, entry: &Entry) -> anyhow::Result<()> {
        let dur = entry.created_at.duration_since(UNIX_EPOCH)?;
        let dur1 = entry.modified_at.duration_since(UNIX_EPOCH)?;

        self.write_multiple_data(vec![
            dur.as_secs() as u32,
            dur.as_nanos() as u32,
            dur1.as_secs() as u32,
            dur.as_nanos() as u32,
            entry.dev,
            entry.inode,
            entry.mode,
            entry.uid,
            entry.gid,
            entry.size,
        ])?;

        self.writer.write_all(&entry.hash)?;
        self.writer.write_u16::<BigEndian>(set_flags(&entry))?;
        self.writer.write_all(entry.name.as_bytes())?;

        Ok(())
    }

    fn pad_entry(&mut self, wrote: u32) -> anyhow::Result<()> {
        let pad_len = 8 - wrote % 8;
        let mut buf = Vec::with_capacity(pad_len as usize);
        buf.extend(std::iter::repeat(b'\x00').take(pad_len as usize));

        self.writer.write_all(buf.as_slice())?;
        Ok(())
    }

    fn encode_footer(&mut self, index: &mut Index) -> anyhow::Result<()> {
        Ok(())
    }

    fn write_multiple_data(&mut self, data: Vec<u32>) -> anyhow::Result<()> {
        for d in data {
            self.writer.write_u32::<BigEndian>(d)?;
        }
        Ok(())
    }
}

fn set_flags(entry: &Entry) -> u16 {
    let mut flags = (entry.stage & 0x3) << 12;
    let name_len: u16 = entry.name.len() as u16;
    if name_len < ENTRY_NAME_MASK {
        flags |= name_len;
    } else {
        flags |= ENTRY_NAME_MASK;
    }

    flags
}
