//! ANSI color helpers.
//!
//! Mirrors `framex/utils/_colors.py` so the Rust CLI output is byte-identical
//! to the Python one. Kept dependency-free on purpose.
//!
//! Color semantics across the CLI:
//! - cyan: path to the dataset
//! - green: saved
//! - yellow: overwritten / warning
//! - magenta: hint (use --overwrite or -o)
//! - red: errors

macro_rules! color {
    ($name:ident, $code:literal) => {
        // The full palette mirrors `_colors.py`; not every color is used.
        #[allow(dead_code)]
        pub fn $name(text: impl std::fmt::Display) -> String {
            format!(concat!("\x1b[", $code, "m{}\x1b[0m"), text)
        }
    };
}

color!(black, "30");
color!(red, "31");
color!(green, "32");
color!(yellow, "33");
color!(blue, "34");
color!(magenta, "35");
color!(cyan, "36");
color!(white, "37");

/// Bold text, matching the Python `\033[1m...\033[22m` pair.
pub fn bold(text: impl std::fmt::Display) -> String {
    format!("\x1b[1m{text}\x1b[22m")
}
