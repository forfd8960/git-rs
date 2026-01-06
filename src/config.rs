use anyhow::Result;
use configparser::ini::Ini;
use std::{collections::HashMap, fs::OpenOptions, io::Write};

use crate::errors::GitError;

#[derive(Debug, Default)]
pub struct Config {
    pub core: Core,
    pub user: Option<User>,
    pub branches: HashMap<String, Branch>,
    pub remotes: HashMap<String, Remote>,
    pub git_path: String,
}

#[derive(Debug, Default)]
pub struct Core {
    pub is_bare: bool,
    pub worktree: String,
    pub filemode: bool,
    pub logallrefupdates: bool,
    pub ignorecase: bool,
    pub precomposeunicode: bool,
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

#[derive(Debug, Default)]
pub struct Remote {
    pub name: String,
    pub url: String,
    pub fetch: String,
}

impl Remote {
    pub fn new(name: String, url: String, fetch: String) -> Self {
        Remote { name, url, fetch }
    }

    pub fn from_map(name: String, remote_map: &HashMap<String, Option<String>>) -> Self {
        Remote {
            name,
            url: remote_map
                .get("url")
                .unwrap_or(&Some("".to_string()))
                .as_deref()
                .unwrap_or("")
                .to_string(),
            fetch: remote_map
                .get("fetch")
                .unwrap_or(&Some("".to_string()))
                .as_deref()
                .unwrap_or("")
                .to_string(),
        }
    }
}

impl Config {
    pub fn new(
        core: Core,
        user: Option<User>,
        branches: HashMap<String, Branch>,
        remotes: HashMap<String, Remote>,
    ) -> Self {
        Config {
            core,
            user,
            branches,
            remotes,
            git_path: "".to_string(),
        }
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

    pub fn get_user(&self) -> Result<&User, GitError> {
        match &self.user {
            Some(u) => Ok(u),
            None => Err(GitError::MissingUser),
        }
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

    // parse config content from string
    pub fn load(&mut self) -> Result<(), GitError> {
        let mut config = Ini::new();
        let conf_map = config
            .load(self.git_path.clone() + "/config")
            .map_err(|e| GitError::LoadConfigFailed(e.to_string()))?;

        // Read core settings
        if let Some(core_map) = conf_map.get("core") {
            self.core = Core::from(core_map);
        }

        // Read user settings
        if let Some(user) = conf_map.get("user") {
            self.user = Some(User::from_map(user));
        }

        // Read remote origin settings
        for (section, section_map) in &conf_map {
            if section.starts_with("branch ") {
                let branch_name = section
                    .trim_start_matches("branch ")
                    .trim_matches('"')
                    .to_string();
                let branch = Branch::from_map(branch_name.clone(), section_map);
                self.branches.insert(branch_name, branch);
            }

            if section.starts_with("remote") {
                let remote_name = section
                    .trim_start_matches("remote ")
                    .trim_matches('"')
                    .to_string();
                self.remotes.insert(
                    remote_name.clone(),
                    Remote::from_map(remote_name.clone(), section_map),
                );
            }
        }

        Ok(())
    }
}

impl Core {
    pub fn from(core_map: &HashMap<String, Option<String>>) -> Self {
        Core {
            is_bare: core_map
                .get("bare")
                .unwrap_or(&Some("false".to_string()))
                .as_deref()
                .unwrap_or("")
                == "true",
            worktree: core_map
                .get("worktree")
                .unwrap_or(&Some("".to_string()))
                .as_deref()
                .unwrap_or("")
                .to_string(),
            filemode: core_map
                .get("filemode")
                .unwrap_or(&Some("false".to_string()))
                .as_deref()
                .unwrap_or("")
                == "true",
            logallrefupdates: core_map
                .get("logallrefupdates")
                .unwrap_or(&Some("false".to_string()))
                .as_deref()
                .unwrap_or("")
                == "true",
            ignorecase: core_map
                .get("ignorecase")
                .unwrap_or(&Some("false".to_string()))
                .as_deref()
                .unwrap_or("")
                == "true",
            precomposeunicode: core_map
                .get("precomposeunicode")
                .unwrap_or(&Some("false".to_string()))
                .as_deref()
                .unwrap_or("")
                == "true",
            repository_format_version: core_map
                .get("repositoryformatversion")
                .unwrap_or(&Some("".to_string()))
                .as_deref()
                .unwrap_or("")
                .to_string(),
        }
    }

    pub fn encode(&self) -> String {
        format!(
            "[core]\n\tbare = {}\n\tworktree = {}\n\tfilemode = {}\n\trepositoryFormatVersion = {}\n",
            self.is_bare, self.worktree, self.filemode, self.repository_format_version
        )
    }
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        User { name, email }
    }

    pub fn from_map(user_map: &HashMap<String, Option<String>>) -> Self {
        User {
            name: user_map
                .get("name")
                .unwrap_or(&Some("".to_string()))
                .as_deref()
                .unwrap_or("")
                .to_string(),
            email: user_map
                .get("email")
                .unwrap_or(&Some("".to_string()))
                .as_deref()
                .unwrap_or("")
                .to_string(),
        }
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

    pub fn from_map(branch_name: String, branch_map: &HashMap<String, Option<String>>) -> Self {
        Branch {
            name: branch_name,
            remote: branch_map
                .get("remote")
                .unwrap_or(&Some("".to_string()))
                .as_deref()
                .unwrap_or("")
                .to_string(),
            merge: branch_map
                .get("merge")
                .unwrap_or(&Some("".to_string()))
                .as_deref()
                .unwrap_or("")
                .to_string(),
            rebase: branch_map
                .get("rebase")
                .unwrap_or(&Some("".to_string()))
                .as_deref()
                .unwrap_or("")
                .to_string(),
            description: branch_map
                .get("description")
                .unwrap_or(&Some("".to_string()))
                .as_deref()
                .unwrap_or("")
                .to_string(),
        }
    }

    pub fn encode(&self) -> String {
        let mut config = String::new();
        config.push_str(&format!("[branch \"{}\"]\n", self.name));

        if self.remote != "" {
            config.push_str(&format!("\tremote = {}\n", self.remote));
        }
        if self.merge != "" {
            config.push_str(&format!("\t\nmerge = {}\n", self.merge));
        }
        if self.rebase != "" {
            config.push_str(&format!("\t\nrebase = {}\n", self.rebase));
        }
        if self.description != "" {
            config.push_str(&format!("\t\ndescription = {}\n", self.description));
        }

        config
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::config::Config;

    #[test]
    fn test_config_load() -> anyhow::Result<()> {
        let mut conf = Config::default();
        let git_path = env::var("GIT_TEST_PATH").unwrap_or_else(|_| "./.git".to_string());
        conf.git_path = git_path;
        conf.load()?;

        let user = conf.user.as_ref().unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john.doe@acme.org");

        println!("{:#?}", conf);
        Ok(())
    }
}
