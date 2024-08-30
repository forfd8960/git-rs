use anyhow::Result;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

#[derive(Debug)]
pub struct Config {
    pub core: Core,
    pub user: Option<User>,
    pub branches: HashMap<String, Branch>,
}

#[derive(Debug)]
pub struct Core {
    pub is_bare: bool,
    pub worktree: String,
    pub comment_char: String,
    pub repository_format_version: String,
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub struct Branch {
    pub name: String,
    pub remote: String,
    pub merge: String,
    pub rebase: String,
    pub description: String,
}

impl Config {
    pub fn new(core: Core, user: Option<User>, branches: HashMap<String, Branch>) -> Self {
        Config {
            core,
            user,
            branches,
        }
    }

    pub fn load(&mut self, path: &str) -> Result<String> {
        let mut file = File::open(path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn init(&self, path: &str) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;

        file.write_all(self.encode().as_bytes())?;
        Ok(())
    }

    pub fn encode(&self) -> String {
        let mut config = String::new();

        config.push_str(&self.core.encode());
        if let Some(u) = &self.user {
            config.push_str(&u.encode());
        }

        for (_, br) in &self.branches {
            config.push_str(br.encode().as_str());
        }

        config
    }

    pub fn decode(&mut self, content: String) -> Result<()> {
        todo!()
    }
}

impl Core {
    pub fn new(
        is_bare: bool,
        worktree: String,
        comment_char: String,
        repository_format_version: String,
    ) -> Self {
        Core {
            is_bare,
            worktree,
            comment_char,
            repository_format_version,
        }
    }

    pub fn encode(&self) -> String {
        format!(
            "[core]\n\tbare = {}\n\tworktree = {}\n\tcommentChar = {}\n\trepositoryFormatVersion = {}\n",
            self.is_bare, self.worktree, self.comment_char, self.repository_format_version
        )
    }
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        User { name, email }
    }

    pub fn encode(&self) -> String {
        format!("[user]\n\tname = {}\n\temail = {}\n", self.name, self.email)
    }
}

impl Branch {
    pub fn new(
        name: String,
        remote: String,
        merge: String,
        rebase: String,
        description: String,
    ) -> Self {
        Branch {
            name,
            remote,
            merge,
            rebase,
            description,
        }
    }

    pub fn encode(&self) -> String {
        format!(
            "[branch \"{}\"]\nremote = {}\nmerge = {}\nrebase = {}\ndescription = {}",
            self.name, self.remote, self.merge, self.rebase, self.description
        )
    }
}
