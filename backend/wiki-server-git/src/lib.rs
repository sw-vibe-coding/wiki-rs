mod ops;

use git2::{Repository, Signature};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use wiki_common::model::WikiPage;

/// Git-backed wiki storage. Each page is a `.md` file in a git repo.
/// Every save/delete creates a commit.
pub struct GitStorage {
    repo: Mutex<Repository>,
    work_dir: PathBuf,
}

impl GitStorage {
    pub fn new(repo_path: &Path, seed: Option<WikiPage>) -> Self {
        let repo = open_or_init_repo(repo_path);
        let storage = Self {
            work_dir: repo_path.to_path_buf(),
            repo: Mutex::new(repo),
        };
        if let Some(page) = seed {
            let path = storage.page_path(&page.title);
            if !path.exists() {
                std::fs::write(&path, &page.content).ok();
                storage.commit(&format!("Add {}", page.title));
            }
        }
        storage
    }

    pub(crate) fn page_path(&self, title: &str) -> PathBuf {
        let safe: String = title
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect();
        self.work_dir.join(format!("{safe}.md"))
    }

    pub(crate) fn commit(&self, message: &str) {
        let repo = self.repo.lock().unwrap();
        let mut index = repo.index().unwrap();
        index
            .add_all(["*.md"], git2::IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree_oid = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_oid).unwrap();
        let sig = signature();
        let parent = repo.head().ok().and_then(|h| h.peel_to_commit().ok());
        let parents: Vec<&git2::Commit> = parent.iter().collect();
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
            .unwrap();
    }
}

fn open_or_init_repo(path: &Path) -> Repository {
    std::fs::create_dir_all(path).ok();
    Repository::open(path).unwrap_or_else(|_| Repository::init(path).unwrap())
}

fn signature() -> Signature<'static> {
    Signature::now("wiki-rs", "wiki-rs@localhost").unwrap()
}
