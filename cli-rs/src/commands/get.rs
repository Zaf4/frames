//! `fx get` — download dataset(s) and save them in a chosen format.
//!
//! Mirrors `_get` from `framex/cli/_cli.py`.

use std::path::PathBuf;

use crate::catalog;
use crate::colors::{bold, cyan, green, magenta, red, yellow};
use crate::constants::local_dir;
use crate::errors::{FramexError, Result};
use crate::http;
use crate::load::{read_ipc_bytes, save};

/// Get a single dataset by `name`.
pub fn run(
    name: &str,
    dir: Option<&str>,
    format: &str,
    overwrite: bool,
    cache: bool,
) -> Result<()> {
    // Resolve the destination directory.
    let destination: PathBuf = if dir.is_none() && !cache {
        std::env::current_dir()?
    } else if cache {
        if dir.is_some() {
            println!(
                "{}",
                yellow(format!(
                    "Both `{}` and `{}` used, ignoring {}.",
                    bold("--dir"),
                    bold("--cache"),
                    bold("--dir")
                ))
            );
        }
        local_dir()?
    } else {
        PathBuf::from(dir.unwrap())
    };

    // Check availability against the remote datasets.
    let remote = catalog::remote_datasets()?;
    let url = remote.get(name).ok_or_else(|| {
        FramexError::DatasetNotFound(red(format!("Dataset `{}` not found.", bold(name))))
    })?;
    let bytes = http::get_bytes(url)?;
    let mut frame = read_ipc_bytes(bytes)?;

    if !destination.exists() {
        let msg = red(format!(
            "Directory `{}` does not exist.",
            bold(destination.display())
        ));
        return Err(FramexError::DirNotFound(msg));
    }

    let path = destination.join(format!("{name}.{format}"));

    if path.exists() {
        if !overwrite {
            let mut msg = format!(
                "Dataset `{}` already exists at `{}`.\n",
                cyan(bold(name)),
                cyan(path.display())
            );
            msg.push_str(&magenta(format!(
                "Use {} or {} to overwrite ",
                bold("--overwrite"),
                bold("-o")
            )));
            msg.push_str(&magenta(format!(
                "or use {} or {} to specify a different directory.",
                bold("-dir"),
                bold("-d")
            )));
            return Err(FramexError::DatasetExists(msg));
        }
        save(&mut frame, &path, format)?;
        println!("{} {}", yellow(bold("Overwritten:")), cyan(path.display()));
    } else {
        save(&mut frame, &path, format)?;
        println!("{} {}", green(bold("Saved:")), cyan(path.display()));
    }

    Ok(())
}
