//! Dataset discovery: remote (GitHub) and local cache.
//!
//! Mirrors `framex/_dicts/_github.py`, `_local.py`, and `_all.py`.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use crate::constants::{EXTENSION, api_url, local_dir};
use crate::errors::{FramexError, Result};
use crate::http;

/// One entry in the GitHub "contents" API listing.
#[derive(Deserialize)]
struct GithubFile {
    name: String,
    download_url: Option<String>,
}

/// Below one hour: reuse the cached `datasets.json`; above: refresh from the API.
const CACHE_TTL_SECS: u64 = 3600;

/// Path to the cached dataset listing: `~/.cache/framex/datasets.json`.
fn cache_json_path() -> Result<PathBuf> {
    let parent = local_dir()?
        .parent()
        .ok_or_else(|| FramexError::Io(std::io::Error::other("no cache parent")))?
        .to_path_buf();
    Ok(parent.join("datasets.json"))
}

/// Names of the remote datasets mapped to their download URLs.
///
/// Mirrors `_get_names`: keep only `*.feather` files, strip the extension.
fn fetch_remote_names() -> Result<HashMap<String, String>> {
    let body = http::get_string(&api_url())?;
    let files: Vec<GithubFile> = serde_json::from_str(&body)?;

    let suffix = format!(".{EXTENSION}");
    let datasets = files
        .into_iter()
        .filter_map(|file| {
            let url = file.download_url?;
            let stem = file.name.strip_suffix(&suffix)?;
            Some((stem.to_string(), url))
        })
        .collect();
    Ok(datasets)
}

/// Remote datasets, using the local JSON cache when it is fresh (< 1 hour).
///
/// Mirrors `_cache_or_remote`.
pub fn remote_datasets() -> Result<HashMap<String, String>> {
    let json_path = cache_json_path()?;

    if json_path.exists() {
        let modified = json_path
            .metadata()?
            .modified()?
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);

        if now.saturating_sub(modified) < CACHE_TTL_SECS {
            let raw = std::fs::read_to_string(&json_path)?;
            return Ok(serde_json::from_str(&raw)?);
        }
    }

    let datasets = fetch_remote_names()?;
    save_json(&json_path, &datasets)?;
    Ok(datasets)
}

fn save_json(path: &PathBuf, datasets: &HashMap<String, String>) -> Result<()> {
    let pretty = serde_json::to_string_pretty(datasets)?;
    std::fs::write(path, pretty)?;
    Ok(())
}

/// All files in the cache dir, keyed by stem (no extension).
///
/// Mirrors `_LOCAL_CACHES`.
pub fn local_caches() -> Result<HashMap<String, PathBuf>> {
    collect_local(|_| true, true)
}

/// All files in the cache dir, keyed by full file name (with extension).
///
/// Mirrors `_LOCAL_CACHES_EXT`.
pub fn local_caches_ext() -> Result<HashMap<String, PathBuf>> {
    collect_local(|_| true, false)
}

/// Only `*.feather` files in the cache dir, keyed by stem.
///
/// Mirrors `_LOCAL_CACHES_MAIN_EXT`.
pub fn local_caches_main_ext() -> Result<HashMap<String, PathBuf>> {
    let suffix = format!(".{EXTENSION}");
    collect_local(move |name| name.ends_with(&suffix), true)
}

/// Walk the cache directory, keyed by stem or full name.
fn collect_local(
    keep: impl Fn(&str) -> bool,
    by_stem: bool,
) -> Result<HashMap<String, PathBuf>> {
    let dir = local_dir()?;
    let mut map = HashMap::new();
    for entry in std::fs::read_dir(&dir)? {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }
        let file_name = match path.file_name().and_then(|name| name.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };
        if !keep(&file_name) {
            continue;
        }
        let key = if by_stem {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .map(|stem| stem.to_string())
                .unwrap_or_else(|| file_name.clone())
        } else {
            file_name
        };
        map.insert(key, path);
    }
    Ok(map)
}
