//! `fx about` — print information about dataset(s) from the remote info CSV.
//!
//! Mirrors `about(name, mode="print")` from `framex/datasets/info.py`.

use std::io::Cursor;

use polars::prelude::*;

use crate::colors::{bold, red};
use crate::constants::INFO_FILE;
use crate::errors::{FramexError, Result};
use crate::http;

/// Print information about a single dataset by `name`.
pub fn run(name: &str) -> Result<()> {
    let bytes = http::get_bytes(INFO_FILE)?;
    let frame = CsvReadOptions::default()
        .with_has_header(true)
        .into_reader_with_file_handle(Cursor::new(bytes))
        .finish()?;

    // Find the matching row by name (eager — avoids pulling the lazy engine).
    let names = frame.column("name")?.as_materialized_series().str()?.clone();
    let index = (0..names.len()).find(|&row| names.get(row) == Some(name));
    let Some(index) = index else {
        // Match the message the Python CLI prints for `about` (not the inner one).
        let msg = red(format!("Dataset `{}` not found.", bold(name)));
        return Err(FramexError::DatasetNotFound(msg));
    };

    let mut source = String::new();
    for column_name in frame.get_column_names() {
        let series = frame.column(column_name)?.as_materialized_series();
        let value = cell_to_string(series.get(index)?);
        if column_name.as_str() == "source" {
            source = value.clone();
        }
        println!("{:<8}: {}", column_name.to_uppercase(), value);
    }

    let og_name = source.rsplit('/').next().unwrap_or(&source);
    println!("{:<8}: {}", "OG NAME", og_name);

    Ok(())
}

/// Render a cell as a plain string (no surrounding quotes for text values).
fn cell_to_string(value: AnyValue) -> String {
    match value {
        AnyValue::String(text) => text.to_string(),
        AnyValue::StringOwned(text) => text.to_string(),
        AnyValue::Null => "null".to_string(),
        other => other.to_string(),
    }
}
