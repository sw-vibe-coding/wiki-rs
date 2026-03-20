use crate::model::WikiPage;

/// Sync storage backend trait for wiki pages (WASM frontends).
pub trait WikiStorage {
    fn get_page(&self, title: &str) -> Option<WikiPage>;
    fn save_page(&self, page: WikiPage);
    fn delete_page(&self, title: &str);
    fn list_pages(&self) -> Vec<String>;
    fn has_page(&self, title: &str) -> bool;
}
