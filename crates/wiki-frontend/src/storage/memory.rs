use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wiki_common::model::WikiPage;

#[derive(Clone, Debug, PartialEq)]
pub struct MemoryStorage {
    pages: Rc<RefCell<HashMap<String, WikiPage>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        let storage = Self {
            pages: Rc::new(RefCell::new(HashMap::new())),
        };
        // Seed with a MainPage
        storage.save_page(WikiPage {
            title: "MainPage".to_string(),
            content: "Welcome to Wiki-RS!\n\nThis is an ephemeral wiki running entirely in your browser.\n\nTry editing this page or clicking a link like [[SandBox]] to create a new page.".to_string(),
        });
        storage
    }

    pub fn get_page(&self, title: &str) -> Option<WikiPage> {
        self.pages.borrow().get(title).cloned()
    }

    pub fn save_page(&self, page: WikiPage) {
        self.pages.borrow_mut().insert(page.title.clone(), page);
    }

    pub fn delete_page(&self, title: &str) {
        self.pages.borrow_mut().remove(title);
    }

    pub fn list_pages(&self) -> Vec<String> {
        let mut titles: Vec<String> = self.pages.borrow().keys().cloned().collect();
        titles.sort();
        titles
    }

    pub fn has_page(&self, title: &str) -> bool {
        self.pages.borrow().contains_key(title)
    }
}
