//! FrameX CLI — Rust rewrite of `framex/cli/_entry.py`.
//!
//! A light-weight dataset fetching tool. Commands: get, bring, about, list,
//! show, describe.

mod catalog;
mod colors;
mod commands;
mod constants;
mod errors;
mod http;
mod load;
mod paths;

use clap::builder::styling::{Ansi256Color, Color, Style, Styles};
use clap::{ArgAction, CommandFactory, Parser, Subcommand};

use commands::list::Which;
use errors::FramexError;

// Exact `rich_argparse` colors from the Python CLI, by xterm-256 palette index
// (rich emits these as 8-bit `38;5;N` escapes, so the rendered bytes match):
//   argparse.groups  = bold deep_pink2   (197, #ff005f)  -> headers / usage
//   argparse.args    = bold dodger_blue1 (33,  #0087ff)  -> flags & subcommands
//   argparse.metavar = orange_red1       (202, #ff5f00)  -> placeholders / metavars
const DEEP_PINK2: Style = Style::new()
    .bold()
    .fg_color(Some(Color::Ansi256(Ansi256Color(197))));
const DODGER_BLUE1: Style = Style::new()
    .bold()
    .fg_color(Some(Color::Ansi256(Ansi256Color(33))));
const ORANGE_RED1: Style = Style::new().fg_color(Some(Color::Ansi256(Ansi256Color(202))));

/// Help styling, matching the Python `rich_argparse` theme.
const STYLES: Styles = Styles::styled()
    .header(DEEP_PINK2)
    .usage(DEEP_PINK2)
    .literal(DODGER_BLUE1)
    .placeholder(ORANGE_RED1);

#[derive(Parser)]
#[command(
    name = "fx",
    about = concat!("Framex CLI ", env!("CARGO_PKG_VERSION")),
    disable_version_flag = true,
    styles = STYLES
)]
struct Cli {
    /// Show version
    #[arg(short = 'v', long = "version", action = ArgAction::SetTrue)]
    version: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Get dataset(s)
    Get {
        /// Dataset name(s), accepts multiple names
        #[arg(required = true, num_args = 1..)]
        datasets: Vec<String>,
        /// Directory to save the dataset to. Defaults to current directory.
        #[arg(short, long)]
        dir: Option<String>,
        /// Format (`feather`, `parquet`, `csv`, `json`, `ipc`) to save the dataset in. Defaults to csv.
        #[arg(short, long, default_value = "csv")]
        format: String,
        /// Whether to overwrite the dataset if it already exists.
        #[arg(short, long)]
        overwrite: bool,
        /// Whether to save to the local cache directory.
        #[arg(short, long)]
        cache: bool,
    },
    /// Bring dataset(s) from the cache to the current working directory or to a specified directory.
    Bring {
        /// Dataset name(s), accepts multiple names
        #[arg(required = true, num_args = 1..)]
        datasets: Vec<String>,
        /// Directory to save the dataset to. Defaults to current directory.
        #[arg(short, long)]
        dir: Option<String>,
        /// Format (`feather`, `parquet`, `csv`, `json`, `ipc`) to save the dataset in. Defaults to csv.
        #[arg(short, long, default_value = "csv")]
        format: String,
        /// Overwrite an existing dataset with the same name.
        #[arg(short, long)]
        overwrite: bool,
    },
    /// Info about dataset(s)
    About {
        /// Info about dataset(s)
        #[arg(required = true, num_args = 1..)]
        datasets: Vec<String>,
    },
    /// List available datasets
    List {
        /// available datasets names which includes the given string.
        includes: Option<String>,
        /// List local datasets.
        #[arg(short, long, conflicts_with_all = ["remote", "all"])]
        local: bool,
        /// List remote datasets.
        #[arg(short, long, conflicts_with_all = ["local", "all"])]
        remote: bool,
        /// List all datasets (both local and remote).
        #[arg(short, long, conflicts_with_all = ["local", "remote"])]
        all: bool,
    },
    /// Show a preview of a single dataset
    Show {
        /// Dataset name
        dataset: String,
    },
    /// Describe (or summarize) a dataset
    Describe {
        /// Dataset name
        dataset: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.version {
        println!("Framex CLI {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let Some(command) = cli.command else {
        // No command: show help, same as the Python CLI.
        let _ = Cli::command().print_help();
        println!();
        return;
    };

    dispatch(command);
}

fn dispatch(command: Command) {
    match command {
        Command::Get {
            datasets,
            dir,
            format,
            overwrite,
            cache,
        } => {
            for dataset in &datasets {
                match commands::get::run(dataset, dir.as_deref(), &format, overwrite, cache) {
                    Ok(()) => {}
                    // A missing directory is fatal for the whole run, as in Python.
                    Err(err @ FramexError::DirNotFound(_)) => {
                        println!("{err}");
                        return;
                    }
                    Err(err) => print_error(err),
                }
            }
        }

        Command::Bring {
            datasets,
            dir,
            format,
            overwrite,
        } => {
            for dataset in &datasets {
                if let Err(err) = commands::bring::run(dataset, dir.as_deref(), &format, overwrite) {
                    print_error(err);
                }
            }
        }

        Command::About { datasets } => {
            for dataset in &datasets {
                match commands::about::run(dataset) {
                    Ok(()) => println!(),
                    Err(err) => print_error(err),
                }
            }
        }

        Command::List {
            includes,
            local,
            remote,
            all,
        } => {
            let _ = all;
            let which = if remote {
                Which::Remote
            } else if local {
                Which::Local
            } else {
                Which::All
            };
            if let Err(err) = commands::list::run(which, includes.as_deref()) {
                print_error(err);
            }
        }

        Command::Show { dataset } => {
            if let Err(err) = commands::show::run(&dataset) {
                print_error(err);
            }
        }

        Command::Describe { dataset } => {
            if let Err(err) = commands::describe::run(&dataset) {
                print_error(err);
            }
        }
    }
}

/// Print an error using its already-colored message, matching the Python CLI.
fn print_error(err: FramexError) {
    println!("{err}");
}
