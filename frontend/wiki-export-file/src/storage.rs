use std::cell::RefCell;
use std::collections::HashMap;
use wiki_common::model::WikiPage;
use wiki_common::storage::WikiStorage;

/// In-memory storage with export/import support via JSON files.
pub struct ExportFileStorage {
    pub(crate) pages: RefCell<HashMap<String, WikiPage>>,
}

impl ExportFileStorage {
    pub fn new() -> Self {
        let mut pages = HashMap::new();
        let seed = WikiPage::new("MainPage", MAIN_PAGE_CONTENT, wiki_common::time::now());
        pages.insert("MainPage".to_string(), seed);
        Self {
            pages: RefCell::new(pages),
        }
    }
}

impl WikiStorage for ExportFileStorage {
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

This wiki supports **JSON file export/import**.
Use the toolbar above to download or upload your wiki pages.

## Getting Started

- Click a link like [[SandBox]] to create a new page
- Use `[[PageName]]` syntax to link between pages
- Use `[[PageName|display text]]` for aliased links
- Full **Markdown** supported: headings, *italic*, **bold**, `code`, lists

## About

Inspired by the history of wiki engines -- from *WikiWikiWeb* (1995) to modern tools.
See [[WikiHistory]] for more.";
