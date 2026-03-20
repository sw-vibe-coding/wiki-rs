mod storage;

use std::rc::Rc;
use storage::BrowserMemoryStorage;

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
    let storage = Rc::new(BrowserMemoryStorage::new(MAIN_PAGE_CONTENT));
    wiki_ui_app::render_wiki(
        storage,
        "Browser Memory (localStorage, persists across refreshes)",
    );
}
