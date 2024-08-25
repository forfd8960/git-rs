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
