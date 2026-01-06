use chrono::{self, DateTime, FixedOffset, Offset, TimeZone};
use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Write},
};

use flate2::Compression;
use flate2::{read::ZlibDecoder, write::ZlibEncoder};

use crate::{
    errors::GitError,
    plumbing::{
        blob::Blob,
        reference::{ReferenceName, SYM_REF_PREFIX},
    },
};

// const DateFormat = "Mon Jan 02 15:04:05 2006 -0700"
const DATEFORMAT: &str = "%a %b %d %H:%M:%S %Y %z";

pub const OBJ_BLOB_HEADER: &str = "blob";
pub const OBJ_TREE_HEADER: &str = "tree";
pub const OBJ_COMMIT_HEADER: &str = "commit";
pub const OBJ_TAG_HEADER: &str = "tag";
pub const OBJECTS_DIR: &str = "objects";

pub enum ObjectType {
    InvalidObject,
    CommitObject,
    TreeObject,
    BlobObject,
    TagObject,
}

// Signature is used to identify who and when created a commit or tag.
#[derive(Clone, Debug, Default)]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub when: DateTime<FixedOffset>,
}

impl Signature {
    pub fn new(name: String, email: String) -> Self {
        let local_offset = chrono::Local::now().offset().fix();
        let now = chrono::Local::now().with_timezone(&local_offset);
        Signature {
            name,
            email,
            when: now,
        }
    }
    // decode signature:
    // John Doe <john.doe@example.com> 1767268800 +0000
    pub fn decode(sig_data: &str) -> Result<Self, GitError> {
        // Find the email part (between < and >)
        let email_start = sig_data.find('<').ok_or(GitError::InvalidSignature(
            "Missing '<' in signature".to_string(),
        ))?;
        let email_end = sig_data.find('>').ok_or(GitError::InvalidSignature(
            "Missing '>' in signature".to_string(),
        ))?;

        // Extract name (everything before '<', trimmed)
        let name = sig_data[..email_start].trim().to_string();

        // Extract email (between < and >)
        let email = sig_data[email_start + 1..email_end].to_string();

        // Extract timestamp and timezone (everything after '>')
        let time_part = sig_data[email_end + 1..].trim();
        let parts: Vec<&str> = time_part.split_whitespace().collect();

        if parts.len() != 2 {
            return Err(GitError::InvalidSignature(
                "Invalid timestamp format".to_string(),
            ));
        }

        // Parse Unix timestamp
        let timestamp: i64 = parts[0]
            .parse()
            .map_err(|_| GitError::InvalidSignature("Invalid timestamp number".to_string()))?;

        // Parse timezone offset (e.g., "+0000", "-0500")
        let tz_str = parts[1];
        let tz_offset = parse_timezone(tz_str)?;

        // Create DateTime from timestamp and timezone
        let when = tz_offset
            .timestamp_opt(timestamp, 0)
            .single()
            .ok_or(GitError::InvalidSignature("Invalid timestamp".to_string()))?;

        Ok(Signature { name, email, when })
    }

    pub fn encode(&self) -> String {
        format!(
            "{} <{}> {}",
            self.name,
            self.email,
            self.when.format(DATEFORMAT)
        )
    }

    pub fn to_string(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }
}

fn parse_timezone(tz: &str) -> Result<FixedOffset, GitError> {
    if tz.len() != 5 {
        return Err(GitError::InvalidSignature(
            "Timezone must be in format +HHMM or -HHMM".to_string(),
        ));
    }

    let sign = match &tz[0..1] {
        "+" => 1,
        "-" => -1,
        _ => {
            return Err(GitError::InvalidSignature(
                "Timezone must start with + or -".to_string(),
            ))
        }
    };

    let hours: i32 = tz[1..3]
        .parse()
        .map_err(|_| GitError::InvalidSignature("Invalid timezone hours".to_string()))?;
    let minutes: i32 = tz[3..5]
        .parse()
        .map_err(|_| GitError::InvalidSignature("Invalid timezone minutes".to_string()))?;

    let total_seconds = sign * (hours * 3600 + minutes * 60);

    FixedOffset::east_opt(total_seconds)
        .ok_or_else(|| GitError::InvalidSignature("Invalid timezone offset".to_string()))
}

pub fn get_object_type(object_type: &str) -> ObjectType {
    match object_type {
        OBJ_COMMIT_HEADER => ObjectType::CommitObject,
        OBJ_TREE_HEADER => ObjectType::TreeObject,
        OBJ_BLOB_HEADER => ObjectType::BlobObject,
        OBJ_TAG_HEADER => ObjectType::TagObject,
        _ => ObjectType::InvalidObject,
    }
}

pub fn object_type_bytes(object_type: &ObjectType) -> &'static [u8] {
    match object_type {
        ObjectType::CommitObject => OBJ_COMMIT_HEADER.as_bytes(),
        ObjectType::TreeObject => OBJ_TREE_HEADER.as_bytes(),
        ObjectType::BlobObject => OBJ_BLOB_HEADER.as_bytes(),
        ObjectType::TagObject => OBJ_TAG_HEADER.as_bytes(),
        ObjectType::InvalidObject => b"invalid",
    }
}
pub fn object_type_string(object_type: &ObjectType) -> &'static str {
    match object_type {
        ObjectType::CommitObject => OBJ_COMMIT_HEADER,
        ObjectType::TreeObject => OBJ_TREE_HEADER,
        ObjectType::BlobObject => OBJ_BLOB_HEADER,
        ObjectType::TagObject => OBJ_TAG_HEADER,
        ObjectType::InvalidObject => "invalid",
    }
}

pub fn write_blob(content: Vec<u8>, hash_bytes: &[u8]) -> anyhow::Result<String> {
    let hash_str = base16ct::lower::encode_string(hash_bytes);
    let (blob_dir, file_name) = get_obj_path(&hash_str);
    println!("[write_blob] blob dir: {}", blob_dir.clone());
    println!("[write_blob] file_name: {}", file_name.clone());

    fs::create_dir(blob_dir)?;

    let blob = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name.clone())?;
    println!("created file");

    let mut e = ZlibEncoder::new(blob, Compression::default());
    e.write_all(&Blob::encode(content))?;
    e.finish()?;

    Ok(file_name)
}

pub fn write_tree(data: Vec<u8>, hash_bytes: &[u8]) -> Result<String, GitError> {
    let hash_str = base16ct::lower::encode_string(hash_bytes);
    let (tree_dir, file_name) = get_obj_path(&hash_str);

    if check_obj_exists(&hash_str) {
        return Ok(file_name.clone());
    }

    println!("[write_tree] tree dir: {}", tree_dir.clone());
    println!("[write_tree] file_name: {}", file_name.clone());

    fs::create_dir(tree_dir)?;

    let tree = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name.clone())?;

    let tree_header = format!("{} {}\0", OBJ_TREE_HEADER, data.len());
    let mut tree_data = tree_header.as_bytes().to_vec();
    tree_data.extend_from_slice(&data);

    let mut e = ZlibEncoder::new(tree, Compression::default());
    e.write_all(&tree_data)?;
    e.finish()?;

    Ok(file_name)
}

pub fn write_commit(data: Vec<u8>, hash_bytes: &[u8]) -> Result<String, GitError> {
    let hash_str = base16ct::lower::encode_string(hash_bytes);
    let (commit_dir, file_name) = get_obj_path(&hash_str);

    if check_obj_exists(&hash_str) {
        return Ok(file_name.clone());
    }

    println!("[write_commit] commit dir: {}", commit_dir.clone());
    println!("[write_commit] file_name: {}", file_name.clone());

    fs::create_dir(commit_dir)?;

    let commit = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name.clone())?;

    let mut e = ZlibEncoder::new(commit, Compression::default());
    e.write_all(&data)?;
    e.finish()?;

    Ok(file_name)
}

pub fn read_object(hash: &str, obj_path: &str) -> anyhow::Result<Vec<u8>> {
    let (obj_dir, obj_file) = get_obj_path(hash);
    let full_path = obj_path.to_owned() + "/" + &obj_dir + "/" + &obj_file;
    let compressed_data = fs::read(full_path)?;

    let mut d = ZlibDecoder::new(&compressed_data[..]);
    let mut obj_data = Vec::new();
    io::copy(&mut d, &mut obj_data)?;

    Ok(obj_data)
}

// resolve a reference to its hash string
pub fn resolve_reference(git_path: &str, ref_name: ReferenceName) -> Result<String, GitError> {
    if ref_name.0.starts_with(SYM_REF_PREFIX) {
        let target_ref = ref_name.0.replace(SYM_REF_PREFIX, "");
        return get_ref(git_path, &target_ref);
    }

    get_ref(git_path, &ref_name.0)
}

pub fn head_ref(git_path: &str) -> Result<String, GitError> {
    let head_path = git_path.to_owned() + "/HEAD";
    let head_data = fs::read_to_string(head_path)?;

    let head_refs = head_data.trim();
    resolve_reference(git_path, head_refs.into())
}

pub fn set_ref(refs_path: &str, ref_name: &str, hash_str: &str) -> anyhow::Result<()> {
    let full_path = refs_path.to_owned() + "/" + ref_name;
    let mut ref_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(full_path)?;

    ref_file.write_all(hash_str.as_bytes())?;
    ref_file.write_all(b"\n")?;
    Ok(())
}

pub fn get_ref(git_path: &str, ref_name: &str) -> Result<String, GitError> {
    let full_path = git_path.to_owned() + "/" + ref_name;
    let ref_data = fs::read_to_string(full_path)?;
    Ok(ref_data.trim().to_string())
}

fn get_obj_path(hash_str: &str) -> (String, String) {
    let git_path = env::var("GIT_TEST_PATH").unwrap_or(".git".to_string());
    let dir = format!("{}/{}/{}", git_path, OBJECTS_DIR, &hash_str[..2]);
    (dir.clone(), dir + "/" + &hash_str[2..])
}

fn check_obj_exists(hash: &str) -> bool {
    let (_, obj_file) = get_obj_path(hash);
    fs::metadata(obj_file).is_ok()
}

#[cfg(test)]
mod tests {
    use chrono::{FixedOffset, TimeZone};

    use crate::plumbing::reference::ReferenceName;

    #[test]
    fn test_resolve_reference() {
        use crate::plumbing::object::resolve_reference;
        use crate::plumbing::reference::REF_HEAD_PREFIX;
        use std::env;

        let git_path = env::var("GIT_TEST_PATH").unwrap_or_else(|_| "/tmp/git_test".to_string());
        let head_ref = resolve_reference(&git_path, "HEAD".into()).unwrap();
        assert_eq!(head_ref, "ref: refs/heads/new-feature".to_string());

        let master_ref = resolve_reference(
            &git_path,
            ReferenceName::from(format!("{}{}", REF_HEAD_PREFIX, "main").as_str()),
        )
        .unwrap();
        assert_eq!(
            master_ref,
            "56915488f2942031acbef36632381ab5e6c49da2".to_string()
        );
    }

    #[test]
    fn test_head_ref() {
        use crate::plumbing::object::head_ref;
        use std::env;

        let git_path = env::var("GIT_TEST_PATH").unwrap_or_else(|_| "/tmp/git_test".to_string());
        let head_hash = head_ref(&git_path).unwrap();
        assert_eq!(
            head_hash,
            "56915488f2942031acbef36632381ab5e6c49da2".to_string()
        );
    }

    #[test]
    fn test_signature_decode() {
        use crate::plumbing::object::Signature;

        let sig_str = "John Doe <john.doe@example.com> 1627846261 +0200";
        let signature = Signature::decode(sig_str).unwrap();
        assert_eq!(signature.name, "John Doe");
        assert_eq!(signature.email, "john.doe@example.com");

        let date_time = FixedOffset::east_opt(2 * 3600)
            .unwrap()
            .timestamp_opt(1627846261, 0)
            .single()
            .unwrap();
        assert_eq!(signature.when, date_time);
    }
}
