use std::cell::RefCell;
use std::collections::HashMap;
use wiki_common::model::WikiPage;
use wiki_common::storage::WikiStorage;

/// In-memory storage with export/import support via JSON files.
pub struct ExportFileStorage {
    pages: RefCell<HashMap<String, WikiPage>>,
}

impl ExportFileStorage {
    pub fn new() -> Self {
        let mut pages = HashMap::new();
        pages.insert(
            "MainPage".to_string(),
            WikiPage {
                title: "MainPage".to_string(),
                content: MAIN_PAGE_CONTENT.to_string(),
            },
        );
        Self {
            pages: RefCell::new(pages),
        }
    }

    pub fn export_json(&self) -> String {
        let pages: Vec<WikiPage> = self.pages.borrow().values().cloned().collect();
        serde_json::to_string_pretty(&pages).unwrap_or_default()
    }

    pub fn import_json(&self, json: &str) -> Result<usize, String> {
        let pages: Vec<WikiPage> =
            serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {e}"))?;
        let count = pages.len();
        let mut store = self.pages.borrow_mut();
        for page in pages {
            store.insert(page.title.clone(), page);
        }
        Ok(count)
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
