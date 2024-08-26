use std::{
    fs::{self, File},
    io::Read,
    os::unix::fs::MetadataExt,
    path::Path,
};

use crate::plumbing::{
    filemode, hash,
    index::{Entry, Index},
    object::{self, ObjectType},
};

pub struct Worktree {
    root: String,
}

impl Worktree {
    pub fn new(root: String) -> Self {
        Worktree { root }
    }

    pub fn add(&mut self, add_file: &str) -> anyhow::Result<()> {
        let current_dir = self.root.clone();

        let dot_git = current_dir.clone() + "/.git";
        println!("dot_git: {}", dot_git);

        let index_path = dot_git.clone() + "/index";
        let mut index = Index::build(&index_path)?;
        println!("{:?}", index);

        let file_path = current_dir.clone() + "/" + add_file;
        println!("file_path: {}", file_path);

        let mut file = File::open(&file_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        let hash_str = hash::compute_hash(&ObjectType::BlobObject, content.as_ref());
        println!("hash: {}", hash_str);

        let obj_blob_path = object::write_blob(content, &hash_str)?;
        println!("successfully write object to: {}", obj_blob_path);

        let add_file_metadata = file.metadata()?;
        let blob_name = get_filename(&file_path);
        if let Some(e) = index.entry(blob_name) {
            // update entry to index
        } else {
            // add entry to index
            let mut e = Entry::new();
            self.fill_entry(&mut e, blob_name, &hash_str, &add_file_metadata)?;
            index.add(&e);
        }

        // write index to index file
        let index_file = File::open(&index_path)?;
        Index::set(index, index_file)?;

        Ok(())
    }

    fn fill_entry(
        &self,
        e: &mut Entry,
        filename: &str,
        hash: &str,
        metdata: &fs::Metadata,
    ) -> anyhow::Result<()> {
        e.name = filename.to_string();
        e.hash = hash.as_bytes().to_vec();
        e.created_at = metdata.created()?;
        e.modified_at = metdata.modified()?;
        e.size = metdata.size() as u32;
        e.dev = metdata.dev() as u32;
        e.inode = metdata.ino() as u32;

        //todo: set mode from file mode
        e.mode = filemode::REGULAR;
        e.gid = metdata.gid() as u32;
        e.uid = metdata.uid() as u32;
        Ok(())
    }
}

fn get_filename(path: &str) -> &str {
    let path = Path::new(path);
    let filename = path.file_name().unwrap();
    filename.to_str().unwrap()
}
