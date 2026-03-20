use crate::model::WikiPage;

/// Re-export for consumers implementing the trait.
pub use async_trait::async_trait;

/// Async storage backend trait for server-side backends.
#[async_trait]
pub trait AsyncWikiStorage: Send + Sync {
    async fn get_page(&self, title: &str) -> Option<WikiPage>;
    async fn save_page(&self, page: WikiPage);
    async fn delete_page(&self, title: &str);
    async fn list_pages(&self) -> Vec<String>;
    async fn has_page(&self, title: &str) -> bool;
}
