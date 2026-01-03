use crate::plumbing::{
    hash::{self, Hash},
    object,
};

const BLOB_HEADER: &str = "blob";

// Blob is used to store arbitrary data - it is generally a file.
pub struct Blob {
    pub hash: hash::Hash,
    pub size: i64,
    pub data: Vec<u8>,
}

impl Blob {
    pub fn new(hash: Hash, size: i64, data: Vec<u8>) -> Self {
        Blob { hash, size, data }
    }

    pub fn from(obj_dir: &str, hash_str: &str) -> anyhow::Result<Self> {
        let obj_data = object::read_object(hash_str, obj_dir)?;

        let mut blob = Blob {
            hash: Hash::from(hash_str),
            size: 0,
            data: Vec::new(),
        };
        blob.decode(&obj_data)?;

        Ok(blob)
    }

    // data is: "blob " + size(content) + "\0" + content
    pub fn decode(&mut self, data: &[u8]) -> anyhow::Result<()> {
        // check header
        let mut i = 0;
        while data[i] != b' ' {
            i += 1;
        }
        let obj_type = &data[0..i];
        if obj_type != BLOB_HEADER.as_bytes() {
            return Err(anyhow::anyhow!("Not a blob object"));
        }

        i += 1; // skip space

        // read size
        let mut size_bytes: Vec<u8> = Vec::new();
        while data[i] != 0 {
            size_bytes.push(data[i]);
            i += 1;
        }

        let size_str = String::from_utf8(size_bytes)?;
        let size: i64 = size_str.parse()?;

        i += 1; // skip null byte
        let null_index = i - 1;

        let content = &data[null_index + 1..];
        self.size = size;
        self.data = content.to_vec();
        Ok(())
    }

    pub fn encode(data: Vec<u8>) -> Vec<u8> {
        let header = format!("{} {}\0", BLOB_HEADER, data.len());
        let mut blob_bs = header.as_bytes().to_vec();
        blob_bs.extend_from_slice(data.as_slice());
        blob_bs
    }
}
