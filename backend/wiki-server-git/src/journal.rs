use crate::GitStorage;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use wiki_common::async_storage::{AsyncWikiStorage, async_trait};
use wiki_common::model::WikiPage;

enum CommitRequest {
    Save { title: String },
    Delete { title: String },
}

/// Git-backed storage with write-ahead journaling.
///
/// Reads and writes hit the filesystem directly for speed.
/// Git commits are queued to a background worker via an mpsc channel,
/// so CAS coordination runs at filesystem speed while history is
/// eventually consistent.
pub struct JournaledGitStorage {
    work_dir: PathBuf,
    commit_tx: mpsc::UnboundedSender<CommitRequest>,
}

impl JournaledGitStorage {
    pub fn new(repo_path: &Path, seed: Option<WikiPage>) -> Self {
        // Initialize the repo and seed page synchronously via GitStorage
        let git = GitStorage::new(repo_path, seed);
        let work_dir = repo_path.to_path_buf();

        let (tx, mut rx) = mpsc::unbounded_channel::<CommitRequest>();

        // Background worker owns the GitStorage and processes commits
        tokio::spawn(async move {
            while let Some(req) = rx.recv().await {
                let msg = match &req {
                    CommitRequest::Save { title } => format!("Update {title}"),
                    CommitRequest::Delete { title } => format!("Delete {title}"),
                };
                git.commit(&msg);
                tracing::trace!("git commit: {msg}");
            }
            tracing::debug!("journal commit worker shutting down");
        });

        Self {
            work_dir,
            commit_tx: tx,
        }
    }

    fn page_path(&self, title: &str) -> PathBuf {
        let safe: String = title
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect();
        self.work_dir.join(format!("{safe}.md"))
    }
}

#[async_trait]
impl AsyncWikiStorage for JournaledGitStorage {
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
        let _ = self
            .commit_tx
            .send(CommitRequest::Save { title: page.title });
    }

    async fn delete_page(&self, title: &str) {
        std::fs::remove_file(self.page_path(title)).ok();
        let _ = self.commit_tx.send(CommitRequest::Delete {
            title: title.to_string(),
        });
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
