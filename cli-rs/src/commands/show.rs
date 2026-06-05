//! `fx show` — print a preview of a single dataset.
//!
//! Mirrors the `show` branch of `framex/cli/_entry.py`.

use crate::colors::{bold, red};
use crate::errors::{FramexError, Result};
use crate::load::load;

/// Load `name` and print the resulting frame.
pub fn run(name: &str) -> Result<()> {
    match load(name) {
        Ok(frame) => {
            println!("{frame}");
            Ok(())
        }
        Err(FramexError::DatasetNotFound(_)) => Err(FramexError::DatasetNotFound(red(format!(
            "Dataset `{}` not found.",
            bold(name)
        )))),
        Err(other) => Err(other),
    }
}
