use async_trait::async_trait;
use wiki_common::model::WikiPage;

/// Async storage backend trait for the wiki server.
#[async_trait]
pub trait AsyncWikiStorage: Send + Sync {
    async fn get_page(&self, title: &str) -> Option<WikiPage>;
    async fn save_page(&self, page: WikiPage);
    async fn delete_page(&self, title: &str);
    async fn list_pages(&self) -> Vec<String>;
    async fn has_page(&self, title: &str) -> bool;
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum BackendKind {
    File,
    Db,
    Git,
}
