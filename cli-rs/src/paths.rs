//! Path resolution matching Python's `pathlib.Path.resolve()`.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// Resolve `path` to an absolute, symlink-canonicalized path.
///
/// Mirrors `Path(dir).resolve()` (non-strict): the path is made absolute and
/// the longest existing prefix is canonicalized, with any missing trailing
/// components re-appended. Unlike `std::fs::canonicalize`, this does not
/// require the full path to exist.
pub fn resolve_dir(path: &Path) -> PathBuf {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|cwd| cwd.join(path))
            .unwrap_or_else(|_| path.to_path_buf())
    };

    let mut trailing: Vec<OsString> = Vec::new();
    let mut current = absolute.as_path();
    loop {
        if let Ok(mut base) = current.canonicalize() {
            for component in trailing.iter().rev() {
                base.push(component);
            }
            return base;
        }
        match (current.file_name(), current.parent()) {
            (Some(name), Some(parent)) => {
                trailing.push(name.to_os_string());
                current = parent;
            }
            // Nothing along the path canonicalizes; fall back to the absolute path.
            _ => return absolute,
        }
    }
}
