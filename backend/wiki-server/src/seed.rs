use crate::storage::BackendKind;
use std::path::Path;
use std::sync::Arc;
use wiki_common::async_storage::AsyncWikiStorage;
use wiki_common::model::WikiPage;

pub fn seed_main_page(backend: &BackendKind) -> WikiPage {
    let desc = match backend {
        BackendKind::File => "**flat file storage**",
        BackendKind::Db => "a **SQLite database**",
        BackendKind::Git => "a **git repository**",
    };
    WikiPage {
        title: "MainPage".to_string(),
        content: format!(
            "# Welcome to Wiki-RS!\n\n\
             This wiki is backed by {desc} on the server.\n\n\
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
        BackendKind::Db => {
            let db_path = data_dir.join("wiki.db");
            let s = wiki_server_db::DbStorage::new(&db_path, Some(seed));
            Arc::new(s)
        }
        BackendKind::Git => todo!("Git backend not yet implemented"),
    }
}
