use crate::GitStorage;
use wiki_common::async_storage::{AsyncWikiStorage, async_trait};
use wiki_common::model::WikiPage;

#[async_trait]
impl AsyncWikiStorage for GitStorage {
    async fn get_page(&self, title: &str) -> Option<WikiPage> {
        let path = self.page_path(title);
        let content = std::fs::read_to_string(&path).ok()?;
        let mtime = wiki_common::time::file_mtime(&path);
        Some(WikiPage {
            title: title.to_string(),
            content,
            created_at: mtime,
            updated_at: mtime,
        })
    }

    async fn save_page(&self, page: WikiPage) {
        std::fs::write(self.page_path(&page.title), &page.content).ok();
        self.commit(&format!("Update {}", page.title));
    }

    async fn delete_page(&self, title: &str) {
        std::fs::remove_file(self.page_path(title)).ok();
        self.commit(&format!("Delete {title}"));
    }

    async fn list_pages(&self) -> Vec<String> {
        let mut titles = Vec::new();
        let Ok(entries) = std::fs::read_dir(&self.work_dir) else {
            return titles;
        };
        for entry in entries.flatten() {
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
