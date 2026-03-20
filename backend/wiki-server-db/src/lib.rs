mod ops;

use rusqlite::Connection;
use std::sync::Mutex;
use wiki_common::model::WikiPage;

/// SQLite-backed wiki storage.
pub struct DbStorage {
    conn: Mutex<Connection>,
}

impl DbStorage {
    pub fn new(db_path: &std::path::Path, seed: Option<WikiPage>) -> Self {
        std::fs::create_dir_all(db_path.parent().unwrap_or(db_path)).ok();
        let conn = Connection::open(db_path).expect("failed to open SQLite database");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS pages (
                title TEXT PRIMARY KEY,
                content TEXT NOT NULL
            )",
        )
        .expect("failed to create pages table");

        let storage = Self {
            conn: Mutex::new(conn),
        };

        if let Some(page) = seed {
            let exists: bool = storage
                .conn
                .lock()
                .unwrap()
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM pages WHERE title = ?1)",
                    [&page.title],
                    |row| row.get(0),
                )
                .unwrap_or(false);
            if !exists {
                storage
                    .conn
                    .lock()
                    .unwrap()
                    .execute(
                        "INSERT INTO pages (title, content) VALUES (?1, ?2)",
                        [&page.title, &page.content],
                    )
                    .ok();
            }
        }

        storage
    }
}
