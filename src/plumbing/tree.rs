use std::collections::HashMap;
use std::env;

use super::hash::Hash;
use crate::{errors::GitError, plumbing::{filemode, hash, index, object::{self, ObjectType}}};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<'a> {
    pub obj_path: String,
    pub entries: Vec<TreeEntry>,
    pub hash: Hash,
    pub m: HashMap<String, &'a TreeEntry>,
    pub t: HashMap<String, &'a Tree<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    // decode tree:
    // [mode + " " + name + "\0" + sha1 (20 bytes)]*
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
    pub fn encode(&self) -> Vec<u8>{
        let mut encoded: Vec<u8> = Vec::new();

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
        encoded
    }
}

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

    pub fn build_tree(&mut self, idx: &index::Index) -> Result<Hash, GitError> {
        self.build_tree_entries(idx);

        let mut root_tree = self.trees.get("").unwrap().clone();
        Ok(self.copy_tree_to_storage_recursive("", &mut root_tree)?)
    }

    pub fn build_tree_entries(&mut self, idx: &index::Index) {
        self.trees
            .insert("".to_string(), Tree::new(&self.obj_path));

        for e in &idx.entries {
            self.build_index_entry(e);
        }
    }

    fn build_index_entry(&mut self, e: &index::Entry) {
        let parts: Vec<&str> = e.name.split('/').collect();

        let mut fullpath = String::new();
        for part in parts {
            let parent = fullpath.clone();
            if !fullpath.is_empty() {
                fullpath.push('/');
            }
            fullpath.push_str(part);

            self.do_build_tree(e, &parent, &fullpath);
        }
    }

    fn do_build_tree(&mut self, e: &index::Entry, parent: &str, fullpath: &str) {
        if self.trees.contains_key(fullpath) {
            return;
        }

        if self.entries.contains_key(fullpath) {
            return;
        }

        let mut te = TreeEntry {
            name: String::from(fullpath.split('/').last().unwrap()),
            mode: 0,
            hash: Hash::default(),
        };

        if fullpath == e.name {
            te.mode = e.mode;
            te.hash = Hash::from(e.hash.clone());
        } else {
            te.mode = filemode::DIR;
            let subtree = Tree::new(&self.obj_path);
            self.trees.insert(fullpath.to_string(), subtree);
        }

        if let Some(parent_tree) = self.trees.get_mut(parent) {
            parent_tree.entries.push(te.clone());
        }
    }

    fn copy_tree_to_storage_recursive(&mut self, parent: &str, tree: &mut Tree) -> Result<Hash, GitError>{
        // sort tree entries by name
        tree.entries.sort_by(|a, b| a.name.cmp(&b.name));

        for idx in 0..tree.entries.len() {
            // clone the entry to avoid holding an immutable borrow while we mutate the vector
            let entry = tree.entries[idx].clone();
            if entry.mode != filemode::DIR && !entry.hash.is_zero() {
                continue;
            }


            let entry_path = if parent.is_empty() {
                entry.name.clone()
            } else {
                format!("{}/{}", parent, entry.name)
            };

            // take subtree out of the map so we don't hold a mutable borrow into self
            let entry_path_key = entry_path.clone();
            if let Some(mut subtree) = self.trees.remove(&entry_path_key) {
                // it's a tree (owned), recurse without holding a mutable borrow of self.trees
                let subtree_hash =
                    self.copy_tree_to_storage_recursive(&entry_path, &mut subtree)?;

                // put subtree back (if you don't need it afterwards you can skip reinserting)
                self.trees.insert(entry_path_key.clone(), subtree);
                let new_entry = TreeEntry {
                    name: entry.name.clone(),
                    mode: entry.mode,
                    hash: subtree_hash,
                };

                tree.entries[idx] = new_entry;
            }
        }

        let tree_bs = tree.encode();
        println!("tree bytes to write: {:?}", String::from_utf8_lossy(&tree_bs));

        let hash_bytes = hash::compute_hash(&ObjectType::TreeObject, &tree_bs);
        object::write_tree(tree_bs, &hash_bytes)?;
        Ok(Hash::from(hash_bytes))
    }
}

#[cfg(test)]

mod tests {
    use crate::plumbing::{filemode::{DIR, REGULAR}, index::Index};
    use std::env;
    use super::*;

    #[test]
    fn test_tree_encode_decode() -> anyhow::Result<()> {
        let git_path = env::var("GIT_TEST_PATH").unwrap_or_else(|_| "/tmp/git_test".to_string());
        println!("git path: {}", git_path);

        let mut tree = Tree::new(&format!("{}/objects", git_path));
        let entry1 = TreeEntry {
            name: "file1.txt".to_string(),
            mode: REGULAR,
            hash: Hash::from("e965047ad7c57865823c7d992b1d046ea66edf78"),
        };
        let entry2 = TreeEntry {
            name: "subdir".to_string(),
            mode: DIR,
            hash: Hash::from("f0d75db65e3b74f4fdebd93915ea7bcb9d93b407"),
        };
        tree.entries.push(entry1);
        tree.entries.push(entry2);

        let encoded = tree.encode();

        println!("encoded tree bytes: {:?}", String::from_utf8_lossy(&encoded));

        let mut decoded_tree = Tree::new(&format!("{}/objects", git_path));
        decoded_tree.decode(&encoded)?;

        assert_eq!(tree.entries.len(), decoded_tree.entries.len());
        for (e1, e2) in tree.entries.iter().zip(decoded_tree.entries.iter()) {
            assert_eq!(e1.name, e2.name);
            assert_eq!(e1.mode, e2.mode);
            assert_eq!(e1.hash.0, e2.hash.0);
        }
        Ok(())
    }

    #[test]
    fn test_build_tree_entries() {
        let git_path = env::var("GIT_TEST_PATH").unwrap_or_else(|_| "/tmp/git_test".to_string());
        let index_path = format!("{}/index", git_path);
        let idx = Index::from(&index_path).unwrap();
        let mut bth = BuildTreeHelper::new(format!("{}/objects", git_path).as_str());
        bth.build_tree_entries(&idx);

        assert_eq!(bth.trees.len(), 2);
        
        let mut expect_tree = Tree::new(format!("{}/objects", git_path).as_str());
        expect_tree.entries.push(TreeEntry {
            name: "test1.txt".to_string(),
            mode: filemode::REGULAR,
            hash: Hash::from("e965047ad7c57865823c7d992b1d046ea66edf78"),
        });
        expect_tree.entries.push(TreeEntry {
            name: "test1".to_string(),
            mode: filemode::DIR,
            hash: Hash::new([0; 20]),
        });
        expect_tree.entries.push(TreeEntry {
            name: "test2.txt".to_string(),
            mode: filemode::REGULAR,
            hash: Hash::from("69dc851c723505eb19abd6f22d2a65f42370f74d"),
        });
        
        assert_eq!(bth.trees.get("").unwrap(), &expect_tree);

        let mut expect_tree1 = Tree::new(format!("{}/objects", git_path).as_str());
        expect_tree1.entries.push(TreeEntry {
            name: "test1-1.txt".to_string(),
            mode: filemode::REGULAR,
            hash: Hash::from("52290d2c5a64e028d9d2411ae0df3e48a82fb5f2"),
        });

        let test1_tree = bth.trees.get("test1").unwrap();
        println!("test1 tree: {:?}", test1_tree);

        assert_eq!(test1_tree, &expect_tree1);

    }

    #[test]
    fn test_build_tree_helper() -> anyhow::Result<()> {
        let git_path = env::var("GIT_TEST_PATH").unwrap_or_else(|_| "/tmp/git_test".to_string());
        let index_path = format!("{}/index", git_path);
        let idx = Index::from(&index_path)?;

        let mut bth = BuildTreeHelper::new(&format!("{}/objects", git_path));
        let tree_hash = bth.build_tree(&idx)?;

        println!("built tree hash: {:}", tree_hash.to_string());

        Ok(())
    }
}