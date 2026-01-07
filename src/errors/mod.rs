use core::error;
use std::{io, path::StripPrefixError, time::SystemTimeError};

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

    #[error("entry not found: {0}")]
    EntryNotFound(String),

    #[error("missing author information")]
    MissingAuthor,

    #[error("missing user information")]
    MissingUser,

    #[error("load config failed: {0}")]
    LoadConfigFailed(String),

    #[error("invalid commit options: {0}")]
    InvalidCommitOptions(String),

    #[error("invalid commit object: {0}")]
    InvalidCommitObject(String),

    #[error("invalid signature error: {0}")]
    InvalidSignature(String),

    #[error("base16ct error: {0:?}")]
    Base16ctError(base16ct::Error),

    #[error("system time error: {0}")]
    SystemTimeError(#[from] SystemTimeError),

    #[error("blob path error: {0}")]
    BlobPathError(#[from] StripPrefixError),
}
