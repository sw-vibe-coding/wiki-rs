use std::path::{Path, PathBuf};
use wiki_common::model::WikiPage;

/// Flat-file storage backend. Each page is stored as `{data_dir}/{title}.md`.
pub struct FileStorage {
    pub(crate) data_dir: PathBuf,
}

impl FileStorage {
    pub async fn new(data_dir: &Path, seed: Option<WikiPage>) -> Self {
        tokio::fs::create_dir_all(data_dir).await.ok();
        let storage = Self {
            data_dir: data_dir.to_path_buf(),
        };
        if let Some(page) = seed
            && !storage.page_path(&page.title).exists()
        {
            let _ = tokio::fs::write(storage.page_path(&page.title), &page.content).await;
        }
        storage
    }

    pub(crate) fn page_path(&self, title: &str) -> PathBuf {
        let safe: String = title
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect();
        self.data_dir.join(format!("{safe}.md"))
    }
}
