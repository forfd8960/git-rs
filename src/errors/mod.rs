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
    #[error("invalid index entry stage")]
    InvalidIndexEntryStage,

    #[error("not supported index version")]
    NotSupportedIndexVersion,
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
}
