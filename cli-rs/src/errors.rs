//! Error types.
//!
//! Mirrors `framex/utils/_exceptions.py`. The user-facing variants carry a
//! message that is already colored at the raise site (same pattern as the
//! Python code), so `main` can print them verbatim.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FramexError {
    /// Invalid output format requested. Mirrors `InvalidFormatError`.
    #[error("{0}")]
    InvalidFormat(String),

    /// Dataset not available remotely or locally. Mirrors `DatasetNotFoundError`.
    #[error("{0}")]
    DatasetNotFound(String),

    /// Destination already holds the dataset. Mirrors `DatasetExistsError`.
    #[error("{0}")]
    DatasetExists(String),

    /// Target directory is missing. Mirrors Python's `FileNotFoundError` usage.
    #[error("{0}")]
    DirNotFound(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Polars(#[from] polars::prelude::PolarsError),

    #[error("network error: {0}")]
    Http(String),

    #[error("failed to parse response: {0}")]
    Parse(String),
}

pub type Result<T> = std::result::Result<T, FramexError>;

impl From<ureq::Error> for FramexError {
    fn from(err: ureq::Error) -> Self {
        FramexError::Http(err.to_string())
    }
}

impl From<serde_json::Error> for FramexError {
    fn from(err: serde_json::Error) -> Self {
        FramexError::Parse(err.to_string())
    }
}
