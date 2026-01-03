use crate::plumbing::blob::Blob;

// File represents git file objects.
pub struct File {
    // Name is the path of the file. It might be relative to a tree,
    // depending of the function that generates it.
    pub name: String,
    // Mode is the file mode.
    pub mode: u32,
    // Blob with the contents of the file.
    pub blob: Blob,
}

impl File {
    pub fn new(name: &str, mode: u32, blob: Blob) -> Self {
        File {
            name: name.to_string(),
            mode,
            blob,
        }
    }
}
