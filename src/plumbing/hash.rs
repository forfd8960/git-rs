use super::object::object_type_bytes;
use super::object::ObjectType;
use anyhow::Result;
use sha1_checked::{Digest, Sha1};

// Size defines the amount of bytes the hash yields.
pub const SIZE: u16 = 20;
// HexSize defines the strings size of the hash when represented in hexadecimal.
pub const HEX_SIZE: u16 = 40;

#[derive(Debug)]
pub struct Hash(pub [u8; SIZE as usize]);

impl Hash {
    pub fn new(bytes: [u8; SIZE as usize]) -> Self {
        Hash(bytes)
    }

    pub fn to_string(&self) -> String {
        base16ct::lower::encode_string(&self.0)
    }
}

impl From<&str> for Hash {
    fn from(hash: &str) -> Self {
        let bytes = base16ct::lower::decode_vec(hash).expect("Failed to decode hash");

        let mut hash_bytes = [0u8; SIZE as usize];
        hash_bytes.copy_from_slice(&bytes);
        Hash(hash_bytes)
    }
}

pub fn compute_hash(t: &ObjectType, content: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(object_type_bytes(t));
    hasher.update(b" ");
    hasher.update(format!("{}", content.len()).as_bytes());
    hasher.update(b"\0");
    hasher.update(content);

    let hf = hasher.try_finalize();
    let arr = hf.hash();
    arr.to_vec()
}
