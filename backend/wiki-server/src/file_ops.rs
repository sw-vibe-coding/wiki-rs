use crate::file_backend::FileStorage;
use wiki_common::async_storage::AsyncWikiStorage;
use wiki_common::async_storage::async_trait;
use wiki_common::model::WikiPage;

#[async_trait]
impl AsyncWikiStorage for FileStorage {
    async fn get_page(&self, title: &str) -> Option<WikiPage> {
        let path = self.page_path(title);
        let content = tokio::fs::read_to_string(&path).await.ok()?;
        let mtime = wiki_common::time::file_mtime(&path);
        Some(WikiPage {
            title: title.to_string(),
            content,
            created_at: mtime,
            updated_at: mtime,
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
