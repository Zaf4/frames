//! Tiny synchronous HTTP helpers over `ureq`.

use std::io::Read;

use crate::errors::{FramexError, Result};

/// Fetch a URL and return its raw bytes.
///
/// Uses the streaming body reader so large dataset files are not capped by
/// `ureq`'s convenience-read size limit.
pub fn get_bytes(url: &str) -> Result<Vec<u8>> {
    let mut response = ureq::get(url).call()?;
    let status = response.status().as_u16();
    if status != 200 {
        return Err(FramexError::Http(format!("received status code {status}")));
    }
    let mut buffer = Vec::new();
    response.body_mut().as_reader().read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Fetch a URL and return its body decoded as UTF-8.
pub fn get_string(url: &str) -> Result<String> {
    let bytes = get_bytes(url)?;
    String::from_utf8(bytes).map_err(|err| FramexError::Parse(err.to_string()))
}
