use crate::storage::AsyncWikiStorage;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use wiki_common::model::WikiPage;

/// Flat-file storage backend. Each page is stored as `{data_dir}/{title}.md`.
pub struct FileStorage {
    data_dir: PathBuf,
}

impl FileStorage {
    pub async fn new(data_dir: &Path, seed: Option<WikiPage>) -> Self {
        tokio::fs::create_dir_all(data_dir).await.ok();
        let storage = Self {
            data_dir: data_dir.to_path_buf(),
        };
        if let Some(page) = seed
            && !storage.has_page(&page.title).await
        {
            storage.save_page(page).await;
        }
        storage
    }

    fn page_path(&self, title: &str) -> PathBuf {
        let safe: String = title
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect();
        self.data_dir.join(format!("{safe}.md"))
    }
}

#[async_trait]
impl AsyncWikiStorage for FileStorage {
    async fn get_page(&self, title: &str) -> Option<WikiPage> {
        let content = tokio::fs::read_to_string(self.page_path(title))
            .await
            .ok()?;
        Some(WikiPage {
            title: title.to_string(),
            content,
        })
    }

    async fn save_page(&self, page: WikiPage) {
        let _ = tokio::fs::write(self.page_path(&page.title), &page.content).await;
    }

    async fn delete_page(&self, title: &str) {
        let _ = tokio::fs::remove_file(self.page_path(title)).await;
    }

    async fn list_pages(&self) -> Vec<String> {
        let mut titles = Vec::new();
        let Ok(mut entries) = tokio::fs::read_dir(&self.data_dir).await else {
            return titles;
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Some(name) = entry.file_name().to_str()
                && let Some(title) = name.strip_suffix(".md")
            {
                titles.push(title.to_string());
            }
        }
        titles.sort();
        titles
    }

    async fn has_page(&self, title: &str) -> bool {
        self.page_path(title).exists()
    }
}
