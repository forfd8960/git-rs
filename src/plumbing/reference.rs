pub const REF_PREFIX: &'static str = "refs/";
pub const REF_HEAD_PREFIX: &'static str = concat!("refs/", "heads/");
pub const REF_TAG_PREFIX: &'static str = concat!("refs/", "tags/");
pub const REF_REMOTE_PREFIX: &'static str = concat!("refs/", "remotes/");
pub const SYM_REF_PREFIX: &'static str = "ref: ";

#[derive(Debug)]
pub enum ReferenceName {
    HEAD(String),   // "HEAD"
    MASTER(String), //  "refs/heads/master"
    MAIN(String),   // "refs/heads/main"
}

#[derive(Debug)]
pub enum ReferenceType {
    InvalidReference,
    HashReference,
    SymbolicReference,
}

#[derive(Debug)]
pub struct Reference {
    pub ref_type: ReferenceType,
    pub name: ReferenceName,
    pub hash: Option<Vec<u8>>,
    pub target: Option<ReferenceName>,
}

impl Reference {
    pub fn new_symbolic_ref(name: ReferenceName, target: ReferenceName) -> Self {
        Reference {
            ref_type: ReferenceType::SymbolicReference,
            name: name,
            hash: None,
            target: Some(target),
        }
    }

    pub fn new_hash_ref(name: ReferenceName, hash: Vec<u8>) -> Self {
        Reference {
            ref_type: ReferenceType::HashReference,
            name: name,
            hash: Some(hash),
            target: None,
        }
    }
}
