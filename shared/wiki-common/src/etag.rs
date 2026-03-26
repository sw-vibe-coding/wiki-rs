use sha2::{Digest, Sha256};

/// Compute an HTTP ETag from page content.
/// Returns a quoted SHA-256 hex string, e.g. `"a1b2c3..."`.
pub fn content_etag(content: &str) -> String {
    let hash = Sha256::digest(content.as_bytes());
    format!("\"{:x}\"", hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn etag_is_deterministic() {
        let a = content_etag("hello world");
        let b = content_etag("hello world");
        assert_eq!(a, b);
    }

    #[test]
    fn etag_differs_for_different_content() {
        let a = content_etag("hello");
        let b = content_etag("world");
        assert_ne!(a, b);
    }

    #[test]
    fn etag_is_quoted() {
        let etag = content_etag("test");
        assert!(etag.starts_with('"'));
        assert!(etag.ends_with('"'));
    }
}
