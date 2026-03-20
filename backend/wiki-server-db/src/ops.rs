use crate::DbStorage;
use wiki_common::async_storage::AsyncWikiStorage;
use wiki_common::async_storage::async_trait;
use wiki_common::model::WikiPage;

#[async_trait]
impl AsyncWikiStorage for DbStorage {
    async fn get_page(&self, title: &str) -> Option<WikiPage> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT title, content FROM pages WHERE title = ?1",
            [title],
            |row| {
                Ok(WikiPage {
                    title: row.get(0)?,
                    content: row.get(1)?,
                })
            },
        )
        .ok()
    }

    async fn save_page(&self, page: WikiPage) {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO pages (title, content) VALUES (?1, ?2)",
            [&page.title, &page.content],
        )
        .ok();
    }

    async fn delete_page(&self, title: &str) {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM pages WHERE title = ?1", [title])
            .ok();
    }

    async fn list_pages(&self) -> Vec<String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT title FROM pages ORDER BY title")
            .unwrap();
        stmt.query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect()
    }

    async fn has_page(&self, title: &str) -> bool {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM pages WHERE title = ?1)",
            [title],
            |row| row.get(0),
        )
        .unwrap_or(false)
    }
}
