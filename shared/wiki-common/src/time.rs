/// Returns current Unix timestamp in seconds.
#[cfg(target_arch = "wasm32")]
pub fn now() -> u64 {
    (js_sys::Date::now() / 1000.0) as u64
}

/// Returns current Unix timestamp in seconds.
#[cfg(not(target_arch = "wasm32"))]
pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Returns file modification time as Unix timestamp in seconds.
#[cfg(not(target_arch = "wasm32"))]
pub fn file_mtime(path: &std::path::Path) -> u64 {
    std::fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
