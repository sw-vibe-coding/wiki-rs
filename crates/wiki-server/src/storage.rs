use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;
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

pub fn seed_main_page(backend: &BackendKind) -> WikiPage {
    let storage_desc = match backend {
        BackendKind::File => "**flat file storage**",
        BackendKind::Db => "a **SQLite database**",
        BackendKind::Git => "a **git repository**",
    };
    WikiPage {
        title: "MainPage".to_string(),
        content: format!(
            "# Welcome to Wiki-RS!\n\n\
             This wiki is backed by {storage_desc} on the server.\n\n\
             ## Getting Started\n\n\
             - Click a link like [[SandBox]] to create a new page\n\
             - Use `[[PageName]]` syntax to link between pages\n\
             - Full **Markdown** supported"
        ),
    }
}

pub async fn create_storage(kind: BackendKind, data_dir: &Path) -> Arc<dyn AsyncWikiStorage> {
    let seed = seed_main_page(&kind);
    match kind {
        BackendKind::File => {
            let s = crate::file_backend::FileStorage::new(data_dir, Some(seed)).await;
            Arc::new(s)
        }
        BackendKind::Db => todo!("SQLite backend not yet implemented"),
        BackendKind::Git => todo!("Git backend not yet implemented"),
    }
}
