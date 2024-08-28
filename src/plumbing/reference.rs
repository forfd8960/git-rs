use crate::storage::filesys::reference::ReferenceStore;
use anyhow::Result;
use std::fmt::Display;

use super::hash::Hash;

pub const REF_PREFIX: &'static str = "refs/";
pub const REF_HEAD_PREFIX: &'static str = concat!("refs/", "heads/");
pub const REF_TAG_PREFIX: &'static str = concat!("refs/", "tags/");
pub const REF_REMOTE_PREFIX: &'static str = concat!("refs/", "remotes/");
pub const SYM_REF_PREFIX: &'static str = "ref: ";

#[derive(Debug)]
pub struct ReferenceName(pub String);

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
    pub hash: Option<Hash>,
    pub target: Option<ReferenceName>,
}

impl Reference {
    pub fn new_from_target(n: ReferenceName, target: String) -> Self {
        if target.starts_with(SYM_REF_PREFIX) {
            let t = ReferenceName(target.replace(SYM_REF_PREFIX, ""));
            return Reference::new_symbolic_ref(n, t);
        }

        let t = target.as_ref();
        Reference::new_hash_ref(n, Hash::from(t))
    }

    pub fn new_symbolic_ref(name: ReferenceName, target: ReferenceName) -> Self {
        Reference {
            ref_type: ReferenceType::SymbolicReference,
            name: name,
            hash: None,
            target: Some(target),
        }
    }

    pub fn new_hash_ref(name: ReferenceName, hash: Hash) -> Self {
        Reference {
            ref_type: ReferenceType::HashReference,
            name: name,
            hash: Some(hash),
            target: None,
        }
    }
}

impl ReferenceName {
    pub fn head() -> Self {
        ReferenceName(String::from("HEAD"))
    }

    pub fn master() -> Self {
        ReferenceName(String::from("refs/heads/master"))
    }

    pub fn main() -> Self {
        ReferenceName(String::from("refs/heads/main"))
    }

    pub fn new_branch_reference(branch_name: &str) -> Self {
        ReferenceName(format!("{}{}", REF_HEAD_PREFIX, branch_name))
    }
}

impl Display for ReferenceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn resolve_reference(s: &ReferenceStore, ref_name: &ReferenceName) -> Result<Reference> {
    todo!()
}
