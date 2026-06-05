//! `fx list` — list available datasets, local and/or remote.
//!
//! Mirrors `_print_avail` from `framex/cli/_cli.py`.

use crate::catalog;
use crate::colors::{blue, bold, cyan, green, red, yellow};
use crate::errors::Result;

/// Which datasets to list.
#[derive(Clone, Copy)]
pub enum Which {
    All,
    Remote,
    Local,
}

/// List datasets, optionally filtered to names containing `includes`.
pub fn run(which: Which, includes: Option<&str>) -> Result<()> {
    match which {
        Which::Local => print_local(includes)?,
        Which::Remote => print_remote(includes)?,
        Which::All => {
            print_local(includes)?;
            print_remote(includes)?;
        }
    }
    Ok(())
}

fn print_local(includes: Option<&str>) -> Result<()> {
    let mut names: Vec<String> = catalog::local_caches_ext()?.into_keys().collect();
    sort_filter(&mut names, includes);

    let legend = format!(
        "({}, {}, {}, {})",
        red("feather"),
        blue("parquet"),
        green("csv"),
        yellow("other")
    );
    println!("{} {}", bold("Locally available datasets:"), legend);

    for name in &names {
        let no_ext = name.split('.').next().unwrap_or(name);
        let colored = if name.contains(".feather") {
            red(no_ext)
        } else if name.contains(".parquet") {
            blue(no_ext)
        } else if name.contains(".csv") {
            green(no_ext)
        } else {
            yellow(no_ext)
        };
        print!("{colored}\t");
    }
    println!();
    Ok(())
}

fn print_remote(includes: Option<&str>) -> Result<()> {
    let mut names: Vec<String> = catalog::remote_datasets()?.into_keys().collect();
    sort_filter(&mut names, includes);

    println!("{}", bold("Remote datasets:"));
    for name in &names {
        print!("{}\t", cyan(name));
    }
    println!();
    Ok(())
}

/// Sort case-insensitively and keep only names containing `includes`.
fn sort_filter(names: &mut Vec<String>, includes: Option<&str>) {
    if let Some(needle) = includes {
        names.retain(|name| name.contains(needle));
    }
    names.sort_by_key(|name| name.to_lowercase());
}
