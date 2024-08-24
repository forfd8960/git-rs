use std::fs;
use std::io::Write;
use std::path::Path;

use crate::plumbing;

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
        let files = [plumbing::HEAD, "config", "description"];
        for file in files {
            let sub_file = format!("{}/{}", full_path, file);
            let _ = fs::File::create(sub_file)?;
        }

        let main_ref_path = format!("{}/{}", full_path, plumbing::HEAD);
        Self::set_head(&main_ref_path, "ref: refs/heads/master")?;
        Ok(())
    }

    fn set_head(file: &str, ref_content: &str) -> anyhow::Result<()> {
        let mut ref_f = fs::File::open(file)?;
        let _ = ref_f.write_all(ref_content.as_bytes())?;
        Ok(())
    }
}
