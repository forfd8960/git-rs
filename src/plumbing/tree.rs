use std::collections::HashMap;

use super::hash::Hash;

pub struct Tree<'a> {
    pub entries: Vec<TreeEntry>,
    pub hash: Hash,
    pub m: HashMap<String, &'a TreeEntry>,
    pub t: HashMap<String, &'a Tree<'a>>,
}

pub struct TreeEntry {
    pub name: String,
    pub mode: u32,
    pub hash: Hash,
}
