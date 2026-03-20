use gloo::storage::{LocalStorage, Storage};
use std::cell::RefCell;
use std::collections::HashMap;
use wiki_common::model::WikiPage;
use wiki_common::storage::WikiStorage;

const STORAGE_KEY: &str = "wiki-rs-pages";

/// Browser localStorage-backed storage.
/// Pages persist across refreshes and browser restarts.
pub(crate) struct BrowserMemoryStorage {
    pages: RefCell<HashMap<String, WikiPage>>,
}

impl BrowserMemoryStorage {
    pub(crate) fn new(seed_content: &str) -> Self {
        let pages: HashMap<String, WikiPage> = LocalStorage::get(STORAGE_KEY).unwrap_or_default();

        let storage = Self {
            pages: RefCell::new(pages),
        };

        if storage.list_pages().is_empty() {
            storage.save_page(WikiPage {
                title: "MainPage".to_string(),
                content: seed_content.to_string(),
            });
        }

        storage
    }

    fn persist(&self) {
        let _ = LocalStorage::set(STORAGE_KEY, &*self.pages.borrow());
    }
}

impl WikiStorage for BrowserMemoryStorage {
    fn get_page(&self, title: &str) -> Option<WikiPage> {
        self.pages.borrow().get(title).cloned()
    }

    fn save_page(&self, page: WikiPage) {
        self.pages.borrow_mut().insert(page.title.clone(), page);
        self.persist();
    }

    fn delete_page(&self, title: &str) {
        self.pages.borrow_mut().remove(title);
        self.persist();
    }

    fn list_pages(&self) -> Vec<String> {
        let mut titles: Vec<String> = self.pages.borrow().keys().cloned().collect();
        titles.sort();
        titles
    }

    fn has_page(&self, title: &str) -> bool {
        self.pages.borrow().contains_key(title)
    }
}
