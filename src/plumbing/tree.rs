use std::collections::HashMap;

use crate::plumbing::object;
use super::hash::Hash;

/*
// Tree is basically like a directory - it references a bunch of other trees
// and/or blobs (i.e. files and sub-directories)
type Tree struct {
	Entries []TreeEntry
	Hash    plumbing.Hash

	s storer.EncodedObjectStorer
	m map[string]*TreeEntry
	t map[string]*Tree // tree path cache
}

// GetTree gets a tree from an object storer and decodes it.
func GetTree(s storer.EncodedObjectStorer, h plumbing.Hash) (*Tree, error) {
	o, err := s.EncodedObject(plumbing.TreeObject, h)
	if err != nil {
		return nil, err
	}

	return DecodeTree(s, o)
}

// DecodeTree decodes an encoded object into a *Tree and associates it to the
// given object storer.
func DecodeTree(s storer.EncodedObjectStorer, o plumbing.EncodedObject) (*Tree, error) {
	t := &Tree{s: s}
	if err := t.Decode(o); err != nil {
		return nil, err
	}

	return t, nil
}

// TreeEntry represents a file
type TreeEntry struct {
	Name string
	Mode filemode.FileMode
	Hash plumbing.Hash
}
*/

#[derive(Debug, Clone)]
pub struct Tree<'a> {
    pub obj_path: String,
    pub entries: Vec<TreeEntry>,
    pub hash: Hash,
    pub m: HashMap<String, &'a TreeEntry>,
    pub t: HashMap<String, &'a Tree<'a>>,
}

#[derive(Debug, Clone)]
pub struct TreeEntry {
    pub name: String,
    pub mode: u32,
    pub hash: Hash,
}


impl Tree<'_> {
    pub fn new(obj_path: &str) -> Self {
        Tree {
            obj_path: obj_path.to_string(),
            entries: Vec::new(),
            hash: Hash::default(),
            m: HashMap::new(),
            t: HashMap::new(),
        }
    }
    // load tree from a given tree object hash from objects storage
    pub fn from(&mut self, hash: &str) -> anyhow::Result<()> {
        let tree_obj = object::read_object(hash, &self.obj_path)?;
        self.decode(&tree_obj)?;
        Ok(())
    }

    pub fn decode(&mut self, data: &[u8]) -> anyhow::Result<()> {
        let mut entries: Vec<TreeEntry> = Vec::new();
        let mut i = 0;
        while i < data.len() {
            // read mode
            let mut mode_bytes: Vec<u8> = Vec::new();
            while data[i] != b' ' {
                mode_bytes.push(data[i]);
                i += 1;
            }
            i += 1; // skip space
            let mode_str = String::from_utf8(mode_bytes)?;
            let mode = u32::from_str_radix(&mode_str, 8)?;

            // read name
            let mut name_bytes: Vec<u8> = Vec::new();
            while data[i] != 0 {
                name_bytes.push(data[i]);
                i += 1;
            }
            i += 1; // skip null byte
            let name = String::from_utf8(name_bytes)?;

            // read hash
            let hash_bytes = &data[i..i + 20];
            let hash = Hash::new(hash_bytes.try_into()?);
            i += 20;

            let entry = TreeEntry { name, mode, hash };
            entries.push(entry);
        }

        self.entries = entries;
        Ok(())
    }

    // encode tree to byte array
    pub fn encode(&self) -> anyhow::Result<Vec<u8>> {
        let mut encoded: Vec<u8> = Vec::new();
        // set obj type to tree
        encoded.extend_from_slice(b"tree ");

        // sort tree entries by name
        let mut sorted_entries = self.entries.clone();
        sorted_entries.sort_by(|a, b| a.name.cmp(&b.name));


        for entry in &sorted_entries {
            let mode_str = format!("{:o}", entry.mode);
            encoded.extend_from_slice(mode_str.as_bytes());
            encoded.push(b' ');
            encoded.extend_from_slice(entry.name.as_bytes());
            encoded.push(0); // null byte
            encoded.extend_from_slice(&entry.hash.0);
        }
        Ok(encoded)
    }
}