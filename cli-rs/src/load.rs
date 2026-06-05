//! Reading and writing datasets.
//!
//! Mirrors `_save` from `framex/cli/_cli.py` and `load` from
//! `framex/datasets/core.py`.

use std::io::Cursor;
use std::path::Path;

use polars::prelude::*;

use crate::catalog;
use crate::colors::{bold, red};
use crate::constants::{EXTENSION, local_dir};
use crate::errors::{FramexError, Result};
use crate::http;

/// Save a `DataFrame` to `path` in the requested `format`.
///
/// Mirrors `_save`: feather/ipc and parquet use zstd compression, json is
/// written as newline-delimited JSON.
pub fn save(frame: &mut DataFrame, path: &Path, format: &str) -> Result<()> {
    // Validate before creating the file, so an invalid format leaves no stray
    // empty file behind (matches the Python `_save`).
    if !matches!(format, "feather" | "ipc" | "parquet" | "csv" | "json") {
        let msg = red(format!(
            "Invalid format: {} format must be one of 'feather', 'parquet', 'csv', 'json', 'ipc'",
            bold(format)
        ));
        return Err(FramexError::InvalidFormat(msg));
    }

    let file = std::fs::File::create(path)?;
    match format {
        "feather" | "ipc" => {
            IpcWriter::new(file)
                .with_compression(Some(IpcCompression::ZSTD(Default::default())))
                .finish(frame)?;
        }
        "parquet" => {
            ParquetWriter::new(file)
                .with_compression(ParquetCompression::Zstd(None))
                .finish(frame)?;
        }
        "csv" => {
            CsvWriter::new(file).finish(frame)?;
        }
        "json" => {
            JsonWriter::new(file)
                .with_json_format(JsonFormat::JsonLines)
                .finish(frame)?;
        }
        _ => unreachable!("format validated above"),
    }
    Ok(())
}

/// Read an IPC/feather file from disk into a `DataFrame`.
pub fn read_ipc_path(path: &Path) -> Result<DataFrame> {
    let file = std::fs::File::open(path)?;
    Ok(IpcReader::new(file).finish()?)
}

/// Read an IPC/feather payload held in memory into a `DataFrame`.
pub fn read_ipc_bytes(bytes: Vec<u8>) -> Result<DataFrame> {
    Ok(IpcReader::new(Cursor::new(bytes)).finish()?)
}

/// Load a dataset by name, caching it locally the first time.
///
/// Mirrors `load(name, cache=True, check_local=True)`: prefer a locally
/// cached feather, otherwise download the remote feather and cache it.
pub fn load(name: &str) -> Result<DataFrame> {
    let local_main = catalog::local_caches_main_ext()?;
    let remote = catalog::remote_datasets()?;

    if !local_main.contains_key(name) && !remote.contains_key(name) {
        let msg = red(format!("Dataset {name} not found."));
        return Err(FramexError::DatasetNotFound(msg));
    }

    if let Some(path) = local_main.get(name) {
        return read_ipc_path(path);
    }

    // Not cached: download, cache, return.
    let url = &remote[name];
    let bytes = http::get_bytes(url)?;
    let mut frame = read_ipc_bytes(bytes)?;

    let local_all = catalog::local_caches()?;
    if !local_all.contains_key(name) {
        let path = local_dir()?.join(format!("{name}.{EXTENSION}"));
        save(&mut frame, &path, EXTENSION)?;
    }

    Ok(frame)
}
