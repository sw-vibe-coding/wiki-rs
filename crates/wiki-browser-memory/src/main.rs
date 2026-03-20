use gloo::storage::{LocalStorage, Storage};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wiki_common::model::WikiPage;
use wiki_common::storage::WikiStorage;

const STORAGE_KEY: &str = "wiki-rs-pages";

/// Browser localStorage-backed storage.
/// Pages persist across refreshes and browser restarts.
struct BrowserMemoryStorage {
    pages: RefCell<HashMap<String, WikiPage>>,
}

impl BrowserMemoryStorage {
    fn new() -> Self {
        let pages: HashMap<String, WikiPage> = LocalStorage::get(STORAGE_KEY).unwrap_or_default();

        let storage = Self {
            pages: RefCell::new(pages),
        };

        // Seed MainPage if empty
        if storage.list_pages().is_empty() {
            storage.save_page(WikiPage {
                title: "MainPage".to_string(),
                content: MAIN_PAGE_CONTENT.to_string(),
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

const MAIN_PAGE_CONTENT: &str = "\
# Welcome to Wiki-RS!

This wiki uses **browser localStorage** for persistence.
Pages survive refreshes and browser restarts.

## Getting Started

- Click a link like [[SandBox]] to create a new page
- Use `[[PageName]]` syntax to link between pages
- Use `[[PageName|display text]]` for aliased links
- Full **Markdown** supported: headings, *italic*, **bold**, `code`, lists

## About

Inspired by the history of wiki engines -- from *WikiWikiWeb* (1995) to modern tools.
See [[WikiHistory]] for more.";

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    let storage = Rc::new(BrowserMemoryStorage::new());
    wiki_ui::app::render_wiki(
        storage,
        "Browser Memory (localStorage, persists across refreshes)",
    );
}
