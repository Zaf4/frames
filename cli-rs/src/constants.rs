//! Project-wide constants.
//!
//! Mirrors `framex/_dicts/_constants.py`.

use std::path::PathBuf;

use crate::errors::{FramexError, Result};

/// feather: almost as effective compression as parquet, much faster loading times.
pub const EXTENSION: &str = "feather";

/// GitHub API URL for the dataset directory contents.
pub fn api_url() -> String {
    format!("https://api.github.com/repos/datavil/datasets/contents/{EXTENSION}")
}

/// Remote info CSV describing every dataset.
pub const INFO_FILE: &str = "https://github.com/datavil/datasets/raw/main/datasets_info.csv";

/// Local cache directory: `~/.cache/framex/datasets`, created if missing.
pub fn local_dir() -> Result<PathBuf> {
    let cache = dirs::home_dir()
        .ok_or_else(|| FramexError::Io(std::io::Error::other("could not resolve home directory")))?
        .join(".cache")
        .join("framex")
        .join("datasets");

    if !cache.exists() {
        std::fs::create_dir_all(&cache)?;
    }
    Ok(cache)
}
