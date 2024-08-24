use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("io error")]
    IOError(#[from] io::Error),
    #[error("malformed index signature file")]
    MalformedIndexSignature,
    #[error("unsupported index version")]
    UnsupportedIndexVersion,
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
}
