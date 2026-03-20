use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wiki_common::model::WikiPage;
use wiki_common::storage::WikiStorage;

/// In-memory storage that is lost on page refresh.
struct EphemeralStorage {
    pages: RefCell<HashMap<String, WikiPage>>,
}

impl EphemeralStorage {
    fn new() -> Self {
        let storage = Self {
            pages: RefCell::new(HashMap::new()),
        };
        storage.save_page(WikiPage {
            title: "MainPage".to_string(),
            content: MAIN_PAGE_CONTENT.to_string(),
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

const MAIN_PAGE_CONTENT: &str = "\
# Welcome to Wiki-RS!

This is an **ephemeral wiki** running entirely in your browser.
All pages are lost on refresh.

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
    let storage = Rc::new(EphemeralStorage::new());
    wiki_ui::app::render_wiki(storage, "Ephemeral (in-memory, lost on refresh)");
}
