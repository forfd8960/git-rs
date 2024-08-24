use anyhow::bail;
use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{BufReader, Read};

use crate::errors::GitError;

use super::Index;

const INDEX_SIG: [u8; 4] = [b'D', b'I', b'R', b'C'];
// DecodeVersionSupported = struct{ Min, Max uint32 }{Min: 2, Max: 4}
const INDEX_VERSION_MIN: u32 = 2;
const INDEX_VERSION_MAX: u32 = 4;

pub struct Decoder {
    pub buf_reader: BufReader<Box<dyn Read>>,
}

impl Decoder {
    pub fn new(reader: Box<dyn Read>) -> Self {
        Decoder {
            buf_reader: BufReader::new(reader),
        }
    }

    pub fn decode(&mut self, idx: &mut Index) -> Result<()> {
        let version = self.validate_header()?;
        println!("version: {}", version);
        idx.version = version;
        Ok(())
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.buf_reader.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn validate_header(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.buf_reader.read_exact(&mut buf)?;

        if !buf.eq(&INDEX_SIG) {
            bail!(GitError::MalformedIndexSignature);
        }

        let version = self.buf_reader.read_u32::<BigEndian>()?;
        if version < INDEX_VERSION_MIN || version > INDEX_VERSION_MAX {
            bail!(GitError::UnsupportedIndexVersion);
        }
        Ok(version)
    }
}
