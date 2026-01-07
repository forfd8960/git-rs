use std::{collections::{HashMap, HashSet}, fs, path::{Path, PathBuf}};

use crate::{errors::GitError, plumbing::{hash::compute_hash, index::{self}, object::ObjectType}};

pub struct GitStatus {
    modified: Vec<String>,
    added: Vec<String>,
    deleted: Vec<String>,
}

impl GitStatus {
    pub fn new() -> Self {
        GitStatus {
            modified: Vec::new(),
            added: Vec::new(),
            deleted: Vec::new(),
        }
    }
}

pub fn detect_changes(index: &index::Index, working_dir: &str) -> Result<GitStatus, GitError> {
    let mut status = GitStatus {
        modified: Vec::new(),
        added: Vec::new(),
        deleted: Vec::new(),
    };

    let mut seen_files = HashSet::new();

    // Check tracked files for modifications
    for entry in &index.entries {
        let full_path = Path::new(working_dir).join(&entry.name);

        seen_files.insert(full_path.clone());

        if !full_path.exists() {
            // File deleted
            status.deleted.push(entry.name.clone());
            continue;
        }

        let metadata = fs::metadata(&full_path)?;
        let current_size = metadata.len();

        // Fast path: size changed
        if current_size != entry.size as u64 {
            status.modified.push(entry.name.clone());
            continue;
        }

        // Fast path: mtime unchanged (likely unchanged)
        let mtime = metadata.modified()?;

        let entry_mtime = entry.modified_at;

        if mtime == entry_mtime {
            // Assume unchanged (optimization)
            continue;
        }

        // Slow path: compute hash
        let content = fs::read(&full_path)?;
        let current_hash = compute_hash(&ObjectType::BlobObject, &content);
        if current_hash != entry.hash {
            status.modified.push(entry.name.clone());
        }
    }

    status.added = scan_untracked_files(PathBuf::from(working_dir), &mut seen_files)?;
    Ok(status)
}

fn scan_untracked_files(
    working_dir: PathBuf,
    seen_files: &mut HashSet<std::path::PathBuf>,
) -> Result<Vec<String>, GitError> {
    let mut untracked_files = Vec::new();

    for entry in fs::read_dir(&working_dir)? {
        let entry = entry?;
        let path = entry.path();

        if seen_files.contains(&path) || path.ends_with(".git") {
            continue;
        }

        if path.is_file() {
            let p = path.strip_prefix(&working_dir)?.to_string_lossy().to_string();
            untracked_files.push(p);
        }

        if path.is_dir() {
            let sub_untracked = scan_untracked_files(path, seen_files)?;
            untracked_files.extend(sub_untracked);
        }
    }

    Ok(untracked_files)
}

#[cfg(test)]
mod tests {
    use crate::errors::GitError;
    use crate::plumbing::index::Index;
    use crate::worktree::status::detect_changes;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_detect_change() -> Result<(), GitError>{
        let idx = Index::from(&format!(
            "{}/index",
            env::var("GIT_TEST_PATH").unwrap_or_else(|_| "/tmp/git_test".to_string())
        ))
        .unwrap();
        
        let working_dir = PathBuf::from(
            env::var("GIT_TEST_WT_PATH").unwrap_or_else(|_| "/tmp/git_test".to_string()),
        );

        let status = detect_changes(&idx, working_dir.to_str().unwrap())?;
        println!("Modified files: {:?}", status.modified);
        println!("Added files: {:?}", status.added);
        println!("Deleted files: {:?}", status.deleted);

        assert_eq!(status.modified.len(), 0);
        assert_eq!(status.added.len(), 1);
        assert_eq!(status.deleted.len(), 0);
        assert_eq!(status.added[0], "test3.txt");

        Ok(())
    }
}