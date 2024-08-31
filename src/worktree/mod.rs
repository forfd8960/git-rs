pub mod commit;

use std::{
    fs::{self, File, OpenOptions},
    io::Read,
    os::unix::fs::MetadataExt,
    path::Path,
};

use crate::plumbing::{
    filemode, hash,
    index::{Entry, Index},
    object::{self, ObjectType},
};

const GIT_DIR: &str = "/.git";

pub struct Worktree {
    pub git_dir_path: String,
}

impl Worktree {
    pub fn new(cur_dir: String) -> Self {
        Worktree {
            git_dir_path: cur_dir + GIT_DIR,
        }
    }

    pub fn add(&mut self, add_file: &str) -> anyhow::Result<()> {
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

    fn add_file_to_storage(&self, content: &[u8]) -> anyhow::Result<Vec<u8>> {
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
    ) -> anyhow::Result<()> {
        let index_path = self.git_dir_path.clone() + "/index";
        let mut index = Index::from(&index_path)?;
        println!("{:?}", index);

        let blob_name = get_filename(file_path);
        let entry = index.entry(blob_name);

        match entry {
            Some(_) => {
                self.update_entry(&mut index, blob_name.to_string(), hash_bytes, metadata)?;
            }
            None => {
                let mut e = Entry::new();
                self.fill_entry(&mut e, &blob_name, hash_bytes, metadata)?;
                index.add(&e);
            }
        }

        // write index to index file
        let index_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&index_path)?;

        println!("set index file back to: {}", index_path);
        Index::set(index, index_file)?;

        Ok(())
    }

    fn fill_entry(
        &self,
        e: &mut Entry,
        filename: &str,
        hash_bytes: &[u8],
        metadata: &fs::Metadata,
    ) -> anyhow::Result<()> {
        e.name = filename.to_string();
        e.hash = hash_bytes.to_vec();
        e.created_at = metadata.created()?;
        e.modified_at = metadata.modified()?;
        e.size = metadata.size() as u32;

        self.fill_sys_info(e, metadata);
        Ok(())
    }

    fn update_entry(
        &self,
        idx: &mut Index,
        name: String,
        hash_bytes: &[u8],
        metadata: &fs::Metadata,
    ) -> anyhow::Result<()> {
        let mut entry = Entry::new();

        entry.name = name.to_string();
        entry.hash = hash_bytes.to_vec();
        entry.modified_at = metadata.modified()?;
        entry.size = metadata.size() as u32;

        self.fill_sys_info(&mut entry, metadata);
        idx.update_entry(entry)
    }

    fn fill_sys_info(&self, e: &mut Entry, metadata: &fs::Metadata) {
        e.dev = metadata.dev() as u32;
        e.inode = metadata.ino() as u32;

        //todo: set mode from file mode
        e.mode = filemode::REGULAR;
        e.stage = 0;
        e.gid = metadata.gid() as u32;
        e.uid = metadata.uid() as u32;
    }

    pub fn read_index(&self) -> anyhow::Result<()> {
        let index_path = self.git_dir_path.clone() + "/index";
        let index = Index::from(&index_path)?;
        println!("{}", index);
        Ok(())
    }
}

fn get_filename(path: &str) -> &str {
    let path = Path::new(path);
    let filename = path.file_name().unwrap();
    filename.to_str().unwrap()
}
