use byteorder::{BigEndian, WriteBytesExt};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use super::{Entry, Index, INDEX_SIG};

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
        }
        Ok(())
    }

    fn encode_entry(&mut self, entry: &Entry) -> anyhow::Result<()> {
        Ok(())
    }

    fn encode_footer(&mut self, index: &mut Index) -> anyhow::Result<()> {
        Ok(())
    }
}
