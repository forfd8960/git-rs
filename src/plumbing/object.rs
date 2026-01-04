use chrono::{self, DateTime, FixedOffset, TimeZone, Utc};
use std::{
    env, fs::{self, OpenOptions}, io::{self, Write}
};

use flate2::Compression;
use flate2::{read::ZlibDecoder, write::ZlibEncoder};

use crate::{errors::GitError, plumbing::{blob::Blob, hash::Hash}};

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
#[derive(Clone, Debug)]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub when: DateTime<FixedOffset>,
}

impl Signature {
    pub fn decode(b: &[u8]) -> Self {
        let b_str = String::from_utf8_lossy(b);
        let open = b_str.rfind('<').unwrap_or(0);
        let close_bracket = b_str.rfind('>').unwrap_or(0);

        let name = b_str[..open].trim().to_string();
        let email = b_str[open + 1..close_bracket].to_string();

        let has_time = close_bracket + 2 < b_str.len();
        let when = if has_time {
            let time_str = &b_str[close_bracket + 2..];
            DateTime::parse_from_str(time_str, DATEFORMAT).unwrap()
        } else {
            FixedOffset::east_opt(0)
                .unwrap()
                .with_ymd_and_hms(2016, 11, 08, 0, 0, 0)
                .unwrap()
        };

        Signature { name, email, when }
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

pub fn read_object(hash: &str, obj_path: &str) -> anyhow::Result<Vec<u8>> {
    let (obj_dir, obj_file) = get_obj_path(hash);
    let full_path = obj_path.to_owned() + "/" + &obj_dir + "/" + &obj_file;
    let compressed_data = fs::read(full_path)?;

    let mut d = ZlibDecoder::new(&compressed_data[..]);
    let mut obj_data = Vec::new();
    io::copy(&mut d, &mut obj_data)?;

    Ok(obj_data)
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