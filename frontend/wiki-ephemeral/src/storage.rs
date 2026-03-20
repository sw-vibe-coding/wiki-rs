use std::cell::RefCell;
use std::collections::HashMap;
use wiki_common::model::WikiPage;
use wiki_common::storage::WikiStorage;

/// In-memory storage that is lost on page refresh.
pub(crate) struct EphemeralStorage {
    pages: RefCell<HashMap<String, WikiPage>>,
}

impl EphemeralStorage {
    pub(crate) fn new(seed_content: &str) -> Self {
        let storage = Self {
            pages: RefCell::new(HashMap::new()),
        };
        storage.save_page(WikiPage {
            title: "MainPage".to_string(),
            content: seed_content.to_string(),
        });
        storage
    }
}

impl WikiStorage for EphemeralStorage {
    fn get_page(&self, title: &str) -> Option<WikiPage> {
        self.pages.borrow().get(title).cloned()
    }

    fn save_page(&self, page: WikiPage) {
        self.pages.borrow_mut().insert(page.title.clone(), page);
    }

    fn delete_page(&self, title: &str) {
        self.pages.borrow_mut().remove(title);
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
