//! `fx bring` — copy dataset(s) from the local cache to a directory.
//!
//! Mirrors `_bring` from `framex/cli/_cli.py`. This is a file copy: it brings
//! the requested format when cached, otherwise falls back to whichever cached
//! format is available, in priority order.

use std::path::{Path, PathBuf};

use crate::colors::{bold, cyan, green, magenta, red, yellow};
use crate::constants::local_dir;
use crate::errors::{FramexError, Result};
use crate::paths::resolve_dir;

/// Format fallback order when the requested format is not cached.
const FORMATS: [&str; 5] = ["csv", "feather", "parquet", "ipc", "json"];

/// Bring a single dataset by `name` from the cache.
pub fn run(name: &str, dir: Option<&str>, format: &str, overwrite: bool) -> Result<()> {
    let cache_dir = local_dir()?;

    let new_dir: PathBuf = match dir {
        Some(path) => resolve_dir(Path::new(path)),
        None => resolve_dir(&std::env::current_dir()?),
    };

    // Anything cached under this name? Mirrors `glob(f"{name}.*")`.
    let prefix = format!("{name}.");
    let any_cached = std::fs::read_dir(&cache_dir)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .any(|file_name| file_name.starts_with(&prefix));
    if !any_cached {
        let msg = red(format!(
            "Dataset `{}` not found in the cache directory.",
            bold(name)
        ));
        return Err(FramexError::DatasetNotFound(msg));
    }

    if !FORMATS.contains(&format) {
        let msg = red(format!(
            "Format `{}` not supported. Please choose from {}",
            bold(format),
            bold(format!("{FORMATS:?}"))
        ));
        return Err(FramexError::InvalidFormat(msg));
    }

    // Refuse to clobber the exact-format destination unless overwriting.
    if !overwrite {
        let destination = new_dir.join(format!("{name}.{format}"));
        if destination.exists() {
            let mut msg = format!(
                "Dataset `{}` already exists in `{}`.\n",
                cyan(bold(name)),
                cyan(destination.display())
            );
            msg.push_str(&magenta(format!(
                "Use {} or {} to overwrite ",
                bold("--overwrite"),
                bold("-o")
            )));
            return Err(FramexError::DatasetExists(msg));
        }
    }

    // Prioritize the requested format when it is cached.
    let exact = cache_dir.join(format!("{name}.{format}"));
    if exact.exists() {
        let destination = new_dir.join(format!("{name}.{format}"));
        std::fs::copy(&exact, &destination)?;
        println!("{} {}", green(bold("Brought:")), cyan(destination.display()));
        return Ok(());
    }

    // Otherwise bring whichever cached format is available, in order.
    for fmt in FORMATS {
        let cache_file = cache_dir.join(format!("{name}.{fmt}"));
        if !cache_file.exists() {
            continue;
        }
        let destination = new_dir.join(format!("{name}.{fmt}"));
        let warning = yellow(format!(
            "Format `{}` not found, using `{}` cache",
            bold(format),
            bold(fmt)
        ));

        if !destination.exists() {
            std::fs::copy(&cache_file, &destination)?;
            println!("{warning}");
            println!("{} {}", green(bold("Brought:")), cyan(destination.display()));
            return Ok(());
        } else if overwrite {
            std::fs::copy(&cache_file, &destination)?;
            println!("{warning}");
            println!("{} {}", yellow(bold("Brought:")), cyan(destination.display()));
            return Ok(());
        } else {
            let mut msg = format!("{warning}\n");
            msg.push_str(&format!(
                "Dataset `{}` already exists in `{}`.\n",
                cyan(bold(name)),
                cyan(destination.display())
            ));
            msg.push_str(&magenta(format!(
                "Use {} or {} to overwrite ",
                bold("--overwrite"),
                bold("-o")
            )));
            return Err(FramexError::DatasetExists(msg));
        }
    }

    Ok(())
}
