use sha1_checked::{Digest, Sha1};

use super::object::object_type_bytes;
use super::object::ObjectType;

// Size defines the amount of bytes the hash yields.
pub const SIZE: u16 = 20;
// HexSize defines the strings size of the hash when represented in hexadecimal.
pub const HEX_SIZE: u16 = 40;

pub fn compute_hash(t: &ObjectType, content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(object_type_bytes(t));
    hasher.update(b" ");
    hasher.update(format!("{}", content.len()).as_bytes());
    hasher.update(b"\0");
    hasher.update(content);

    let result = hasher.try_finalize();
    base16ct::lower::encode_string(result.hash().as_ref())
}
