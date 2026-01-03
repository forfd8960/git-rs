use std::collections::HashMap;

use super::hash::Hash;
use crate::plumbing::{hash, index, object};

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
        encoded.extend_from_slice((object::OBJ_TREE_HEADER.to_string() + " ").as_bytes());

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

// buildTreeHelper converts a given index.Index file into multiple git objects
// reading the blobs from the given filesystem and creating the trees from the
// index structure. The created objects are pushed to a given Storer.
/*type buildTreeHelper struct {
    fs billy.Filesystem
    s  storage.Storer

    trees   map[string]*object.Tree
    entries map[string]*object.TreeEntry
}*/

pub struct BuildTreeHelper<'a> {
    pub obj_path: String,
    pub trees: HashMap<String, Tree<'a>>,
    pub entries: HashMap<String, TreeEntry>,
}

impl BuildTreeHelper<'_> {
    pub fn new(obj_path: &str) -> Self {
        BuildTreeHelper {
            obj_path: obj_path.to_string(),
            trees: HashMap::new(),
            entries: HashMap::new(),
        }
    }

    pub fn build_tree(&mut self, idx: &index::Index) -> anyhow::Result<Hash> {
        const ROOT_NODE: &str = "";
        self.trees
            .insert(ROOT_NODE.to_string(), Tree::new(&self.obj_path));

        for e in &idx.entries {
            self.commit_index_entry(e)?;
        }

        Ok(self.copy_tree_to_storage_recursive(ROOT_NODE, self.trees.get(ROOT_NODE).unwrap())?)
    }

    fn commit_index_entry(&mut self, e: &index::Entry) -> anyhow::Result<()> {
        let path_parts: Vec<&str> = e.name.split('/').collect();
        let mut current_path = String::new();

        for (i, part) in path_parts.iter().enumerate() {
            if i == path_parts.len() - 1 {
                // last part - file
                let hash = Hash::new(e.hash.as_slice().try_into()?);
                let entry = TreeEntry {
                    name: part.to_string(),
                    mode: e.mode,
                    hash,
                };
                self.entries.insert(e.name.clone(), entry);
            } else {
                // directory
                if !current_path.is_empty() {
                    current_path.push('/');
                }
                current_path.push_str(part);

                if !self.trees.contains_key(&current_path) {
                    self.trees
                        .insert(current_path.clone(), Tree::new(&self.obj_path));
                }
            }
        }
        Ok(())
    }

    fn copy_tree_to_storage_recursive(&self, root: &str, tree: &Tree) -> anyhow::Result<_, Hash> {
        let mut new_tree = Tree::new(&self.obj_path);
        for entry in &tree.entries {
            if let Some(t) = self.trees.get(&format!("{}/{}", root, entry.name)) {
                // it's a tree
                let subtree_hash =
                    self.copy_tree_to_storage_recursive(&format!("{}/{}", root, entry.name), t)?;
                let new_entry = TreeEntry {
                    name: entry.name.clone(),
                    mode: entry.mode,
                    hash: subtree_hash,
                };
                new_tree.entries.push(new_entry);
            } else if let Some(e) = self.entries.get(&format!("{}/{}", root, entry.name)) {
                // it's a blob
                new_tree.entries.push(e.clone());
            }
        }
        let encoded_tree = new_tree.encode()?;
        let hash_bytes = hash::compute_hash(&object::ObjectType::TreeObject, &encoded_tree);
        object::write_tree(encoded_tree, &hash_bytes)?;
        Ok(Hash::from(hash_bytes))
    }
}
