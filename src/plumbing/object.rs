use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};

use flate2::write::ZlibEncoder;
use flate2::Compression;

pub enum ObjectType {
    InvalidObject,
    CommitObject,
    TreeObject,
    BlobObject,
    TagObject,
}

pub fn get_object_type(object_type: &str) -> ObjectType {
    match object_type {
        "commit" => ObjectType::CommitObject,
        "tree" => ObjectType::TreeObject,
        "blob" => ObjectType::BlobObject,
        "tag" => ObjectType::TagObject,
        _ => ObjectType::InvalidObject,
    }
}

pub fn object_type_bytes(object_type: &ObjectType) -> &'static [u8] {
    match object_type {
        ObjectType::CommitObject => b"commit",
        ObjectType::TreeObject => b"tree",
        ObjectType::BlobObject => b"blob",
        ObjectType::TagObject => b"tag",
        ObjectType::InvalidObject => b"invalid",
    }
}
pub fn object_type_string(object_type: &ObjectType) -> &'static str {
    match object_type {
        ObjectType::CommitObject => "commit",
        ObjectType::TreeObject => "tree",
        ObjectType::BlobObject => "blob",
        ObjectType::TagObject => "tag",
        ObjectType::InvalidObject => "invalid",
    }
}

pub fn write_blob(content: Vec<u8>, hash_str: &str) -> anyhow::Result<String> {
    let content_bytes = content;
    let content_len = content_bytes.len();
    let header = format!("blob {}\0", content_len);
    let mut blob_bs = header.as_bytes().to_vec();

    blob_bs.extend_from_slice(content_bytes.as_slice());

    let (blob_dir, file_name) = get_blob_path(hash_str);
    println!("[write_blob] blob dir: {}", blob_dir.clone());
    println!("[write_blob] file_name: {}", file_name.clone());

    fs::create_dir(blob_dir)?;

    let blob = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name.clone())?;
    println!("created file");

    let mut e = ZlibEncoder::new(blob, Compression::default());
    e.write_all(&blob_bs)?;
    e.finish()?;

    Ok(file_name)
}

fn get_blob_path(hash_str: &str) -> (String, String) {
    let dir = ".git/objects/".to_owned() + &hash_str[0..2];
    (dir.clone(), dir + "/" + &hash_str[2..])
}
