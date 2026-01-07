pub mod commit;
pub mod status;

use std::{
    fs::{self, File, OpenOptions},
    io::Read,
    os::unix::fs::MetadataExt,
    path::Path,
};

use crate::{
    errors::GitError,
    plumbing::{
        filemode, hash,
        index::{Entry, Index},
        object::{self, ObjectType},
    },
    worktree::status::detect_changes,
};

const GIT_DIR: &str = "/.git";
const OBJECTS_DIR: &str = "/objects";
const HEAD_FILE: &str = "/HEAD";
const IDX_NAME: &str = "index";

pub struct Worktree {
    pub git_dir_path: String,
}

pub struct FileInfo {
    path: String,
    hash: Vec<u8>,
    metadata: fs::Metadata,
}

impl Worktree {
    pub fn new(cur_dir: String) -> Self {
        Worktree {
            git_dir_path: cur_dir + GIT_DIR,
        }
    }

    pub fn add(&mut self, add_file: &str) -> Result<(), GitError> {
        println!("dot_git: {}", self.git_dir_path);

        let file_path = self.git_dir_path.clone() + "/" + add_file;
        println!("file_path: {}", file_path);

        let mut file = File::open(&file_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        let hash_bytes = self.add_file_to_storage(&content)?;
        let add_file_metadata = file.metadata()?;

        self.add_file_to_index(&file_path, &hash_bytes, &add_file_metadata)
    }

    pub fn add_files(&mut self, files: Vec<&str>) -> Result<(), GitError> {
        if files.len() == 0 {
            return Ok(());
        }

        let mut idx_files = Vec::new();
        for add_file in files {
            let file_path = self.git_dir_path.clone() + "/" + add_file;
            let mut file = File::open(&file_path)?;
            let mut content = Vec::new();
            file.read_to_end(&mut content)?;

            let hash_bytes = self.add_file_to_storage(&content)?;
            let add_file_metadata = file.metadata()?;

            idx_files.push(FileInfo {
                path: file_path,
                hash: hash_bytes,
                metadata: add_file_metadata,
            });
        }

        self.add_files_to_index(idx_files)
    }

    pub fn remove_files(&mut self, files: Vec<&str>) -> Result<(), GitError> {
        if files.len() == 0 {
            return Ok(());
        }

        let mut idx = self.read_index()?;
        for f in files {
            idx.remove_entry(f)?;
        }

        self.write_index(idx)
    }

    fn add_file_to_storage(&self, content: &[u8]) -> Result<Vec<u8>, GitError> {
        let hash_bytes = hash::compute_hash(&ObjectType::BlobObject, content);
        println!("hash: {:?}, len: {}", hash_bytes, hash_bytes.len());

        let obj_blob_path = object::write_blob(content.to_vec(), &hash_bytes)?;
        println!("successfully write object to: {}", obj_blob_path);
        Ok(hash_bytes)
    }

    fn add_file_to_index(
        &self,
        file_path: &str,
        hash_bytes: &[u8],
        metadata: &fs::Metadata,
    ) -> Result<(), GitError> {
        let index_path = self.git_dir_path.clone() + "/" + IDX_NAME;
        println!("index_path: {}", index_path);

        let mut index = Index::from(&index_path)?;
        index.add_file(file_path, hash_bytes, metadata)?;

        // write index to index file
        self.write_index(index)
    }

    pub fn add_files_to_index(&self, files: Vec<FileInfo>) -> Result<(), GitError> {
        let index_path = self.git_dir_path.clone() + "/" + IDX_NAME;
        println!("index_path: {}", index_path);

        let mut index = Index::from(&index_path)?;

        for file in files {
            let file_path = file.path;
            let hash_bytes = file.hash;
            let metadata = file.metadata;
            index.add_file(&file_path, &hash_bytes, &metadata)?;
        }

        // write index to index file
        self.write_index(index)
    }

    pub fn write_index(&self, index: Index) -> Result<(), GitError> {
        let index_path = self.git_dir_path.clone() + "/" + IDX_NAME;

        let index_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&index_path)?;

        println!("set index file back to: {}", index_path);
        Index::set(index, index_file)
    }

    pub fn read_index(&self) -> Result<Index, GitError> {
        let index_path = self.git_dir_path.clone() + "/" + IDX_NAME;
        let index = Index::from(&index_path)?;
        Ok(index)
    }

    pub fn auto_add_modified_and_deleted(&mut self) -> Result<(), GitError> {
        let idx = self.read_index()?;
        let working_dir = self.git_dir_path.replace(GIT_DIR, "");
        let status = detect_changes(&idx, working_dir.as_str())?;

        self.add_files(
            status
                .added()
                .iter()
                .chain(status.modified())
                .map(|f| f.as_str())
                .collect(),
        )?;
        self.remove_files(status.deleted().iter().map(|f| f.as_str()).collect())?;
        Ok(())
    }
}
