use crate::model::WikiPage;

/// Storage backend trait for wiki pages.
///
/// All methods are synchronous since WASM is single-threaded.
/// Server-backed implementations can use internal caching or
/// block on HTTP requests via `gloo::net`.
pub trait WikiStorage {
    fn get_page(&self, title: &str) -> Option<WikiPage>;
    fn save_page(&self, page: WikiPage);
    fn delete_page(&self, title: &str);
    fn list_pages(&self) -> Vec<String>;
    fn has_page(&self, title: &str) -> bool;
}
