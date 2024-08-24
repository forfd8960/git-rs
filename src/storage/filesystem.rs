use std::fs;
use std::io::Write;
use std::path::Path;

use crate::plumbing::{self, reference};

use super::Store;

// The file operations git will perforam.
pub struct FileSystem {
    pub root: String,
}

impl FileSystem {
    pub fn new(path: &str) -> Self {
        FileSystem {
            root: path.to_string(),
        }
    }

    pub(crate) fn create_and_write_file(file_path: &str, content: &str) -> anyhow::Result<()> {
        println!("create: {}, and write: {}", file_path, content);
        let mut file = fs::File::create(file_path)?;
        if content == "" {
            return Ok(());
        }

        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl Store for FileSystem {
    fn init_dot_git(&self) -> anyhow::Result<()> {
        let full_path = format!("{}/{}", self.root, ".git");
        if !Path::new(&full_path).exists() {
            fs::create_dir(full_path.clone())?;
        }

        // create the directories
        let dirs = ["objects", "refs", "hooks", "info"];
        for dir in dirs {
            let sub_dir = format!("{}/{}", full_path, dir);
            if !Path::new(&sub_dir).exists() {
                fs::create_dir(sub_dir.clone())?;
                match dir {
                    "objects" => {
                        fs::create_dir(format!("{}/{}", sub_dir, "info"))?;
                        fs::create_dir(format!("{}/{}", sub_dir, "pack"))?;
                    }
                    "refs" => {
                        fs::create_dir(format!("{}/{}", sub_dir, "heads"))?;
                        fs::create_dir(format!("{}/{}", sub_dir, "tags"))?;
                    }
                    "info" => {
                        fs::create_dir(format!("{}/{}", sub_dir, "exclude"))?;
                    }
                    _ => {}
                }
            }
        }

        // create the files
        let head_file = format!("{}/{}", full_path, plumbing::HEAD);
        
        Self::create_and_write_file(
            &head_file,
            &format!("{}{}\n", reference::SYM_REF_PREFIX, plumbing::MAIN),
        )?;

        let conf_file = format!("{}/{}", full_path, "config");
        Self::create_and_write_file(&conf_file, "")?;

        let desc_file = format!("{}/{}", full_path, "description");
        Self::create_and_write_file(&desc_file, "")?;

        Ok(())
    }
}
