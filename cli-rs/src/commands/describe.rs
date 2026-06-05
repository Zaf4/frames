//! `fx describe` — print summary statistics for a single dataset.
//!
//! Mirrors the `describe` branch of `framex/cli/_entry.py`. Rust `polars` has
//! no data `DataFrame::describe` (it lives in the Python binding), so the
//! summary is built here: numeric columns get f64 statistics, non-numeric
//! columns get string min/max with the rest left null — same shape as the
//! Python output.

use polars::prelude::*;

use crate::colors::{bold, red};
use crate::errors::{FramexError, Result};
use crate::load::load;

/// Statistic rows, in the same order the Python `describe()` produces.
const STATS: [&str; 9] = [
    "count",
    "null_count",
    "mean",
    "std",
    "min",
    "25%",
    "50%",
    "75%",
    "max",
];

/// Load `name` and print its `describe()` summary.
pub fn run(name: &str) -> Result<()> {
    let frame = match load(name) {
        Ok(frame) => frame,
        Err(FramexError::DatasetNotFound(_)) => {
            return Err(FramexError::DatasetNotFound(red(format!(
                "Dataset `{}` not found.",
                bold(name)
            ))));
        }
        Err(other) => return Err(other),
    };

    let summary = describe(&frame)?;
    println!("{summary}");
    Ok(())
}

/// Build a summary frame: a `statistic` column plus one column per input column.
fn describe(frame: &DataFrame) -> PolarsResult<DataFrame> {
    let mut columns: Vec<Column> = Vec::with_capacity(frame.width() + 1);
    columns.push(Column::new("statistic".into(), &STATS));

    for column_name in frame.get_column_names() {
        let series = frame.column(column_name)?.as_materialized_series();
        columns.push(summarize(series));
    }

    DataFrame::new(STATS.len(), columns)
}

/// Summarize a single series into a 9-row column of statistics.
fn summarize(series: &Series) -> Column {
    let name = series.name().clone();
    let count = (series.len() - series.null_count()) as f64;
    let null_count = series.null_count() as f64;

    if series.dtype().is_primitive_numeric() {
        let quantile = |percentile: f64| -> Option<f64> {
            series
                .quantile_reduce(percentile, QuantileMethod::Nearest)
                .ok()
                .and_then(|scalar| scalar.value().extract::<f64>())
        };
        let min = series
            .min_reduce()
            .ok()
            .and_then(|scalar| scalar.value().extract::<f64>());
        let max = series
            .max_reduce()
            .ok()
            .and_then(|scalar| scalar.value().extract::<f64>());

        let values = vec![
            Some(count),
            Some(null_count),
            series.mean(),
            series.std(1),
            min,
            quantile(0.25),
            quantile(0.50),
            quantile(0.75),
            max,
        ];
        Series::new(name, values).into()
    } else {
        // Non-numeric: show string min/max, leave numeric-only stats null.
        let min = series.min_reduce().ok().and_then(scalar_to_string);
        let max = series.max_reduce().ok().and_then(scalar_to_string);

        let values: Vec<Option<String>> = vec![
            Some(count.to_string()),
            Some(null_count.to_string()),
            None,
            None,
            min,
            None,
            None,
            None,
            max,
        ];
        Series::new(name, values).into()
    }
}

/// Render a scalar as a plain string, or `None` when it is null.
fn scalar_to_string(scalar: Scalar) -> Option<String> {
    match scalar.value() {
        AnyValue::Null => None,
        AnyValue::String(text) => Some(text.to_string()),
        AnyValue::StringOwned(text) => Some(text.to_string()),
        other => Some(other.to_string()),
    }
}
