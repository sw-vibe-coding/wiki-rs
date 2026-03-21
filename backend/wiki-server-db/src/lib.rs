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
                content TEXT NOT NULL,
                created_at INTEGER NOT NULL DEFAULT 0,
                updated_at INTEGER NOT NULL DEFAULT 0
            )",
        )
        .expect("failed to create pages table");

        let storage = Self {
            conn: Mutex::new(conn),
        };

        if let Some(page) = seed {
            seed_if_missing(&storage, &page);
        }

        storage
    }
}

fn seed_if_missing(storage: &DbStorage, page: &WikiPage) {
    let conn = storage.conn.lock().unwrap();
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM pages WHERE title = ?1)",
            [&page.title],
            |row| row.get(0),
        )
        .unwrap_or(false);
    if !exists {
        conn.execute(
            "INSERT INTO pages (title, content, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![page.title, page.content, page.created_at, page.updated_at],
        )
        .ok();
    }
}
